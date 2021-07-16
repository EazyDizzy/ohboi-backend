use std::num::ParseIntError;
use std::str::FromStr;

use async_trait::async_trait;
use regex::Regex;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};

use crate::my_enum::CurrencyEnum;
use crate::parse::crawler::crawler::{get_html_nodes, Crawler, ProductHtmlSelectors};
use crate::parse::db::entity::category::CategorySlug;
use crate::parse::db::entity::source::SourceName;
use crate::parse::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::parse::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::parse::dto::parsed_product::{
    AdditionalParsedProductInfo, LocalParsedProduct, TypedCharacteristic,
};
use crate::parse::service::html_cleaner::inner_text;

static SITE_BASE: &str = "https://mi-shop.com";

lazy_static! {
    static ref DESCRIPTION_RE: Regex =
        Regex::new(r"(?ms)<p>.*?</p>|<h2>.*?</h2>|<ul>.*?</ul>").unwrap();
}

#[derive(Clone)]
pub struct MiShopComCrawler {}

#[async_trait(? Send)]
impl Crawler for MiShopComCrawler {
    fn get_source(&self) -> SourceName {
        SourceName::MiShopCom
    }

    fn get_currency(&self) -> CurrencyEnum {
        CurrencyEnum::RUB
    }

    fn get_categories(&self) -> Vec<CategorySlug> {
        vec![
            CategorySlug::Smartphone,
            CategorySlug::SmartHome,
            CategorySlug::Headphones,
            CategorySlug::Watches,
        ]
    }

    fn get_next_page_urls(&self, category: CategorySlug) -> Vec<String> {
        let base = [SITE_BASE, "/ru/catalog/"].concat();
        let pagination = "/page/{page}/";

        let urls = match category {
            CategorySlug::Smartphone => vec!["smartphones"],
            CategorySlug::SmartHome => vec![
                "smart_devices/umnyy-dom",
                "smart_devices/foto-video",
                "smart_devices/osveshchenie",
            ],
            CategorySlug::Headphones => vec!["audio"],
            CategorySlug::Watches => vec!["smart_devices/umnye-chasy-i-braslety"],
        };

        urls.into_iter()
            .map(|url| [&base, url, pagination].concat())
            .collect()
    }

    fn extract_products(&self, document: &Html) -> Vec<LocalParsedProduct> {
        let mut parsed_products = vec![];
        let items_selector = Selector::parse(".js-catalog-item").unwrap();

        let selectors = ProductHtmlSelectors {
            id: Selector::parse("a.product-card__name[href]").unwrap(),
            title: Selector::parse(".product-card__title").unwrap(),
            price: Selector::parse(".price__new").unwrap(),
            available: Selector::parse(".btn-buy").unwrap(),
            unavailable: Selector::parse(".btn-buy.disabled").unwrap(),
        };

        for element in document.select(&items_selector) {
            let nodes = get_html_nodes(&selectors, &element, self.get_source());

            if nodes.is_none() {
                continue;
            }

            let product_nodes = nodes.unwrap();

            let title: String = {
                let mut title_value = product_nodes.title.inner_html();
                if title_value.contains('(') {
                    // removing color information from title
                    title_value = title_value.split('(').next().unwrap().trim().to_string();
                }

                title_value
            };

            let price: f64 = {
                let price_html = product_nodes.price.inner_html();
                let price_text = price_html
                    .replace('₽', "")
                    .replace(' ', "")
                    .trim()
                    .parse::<f64>();

                if price_text.is_err() {
                    let message = format!(
                        "price_text({html}) can't be parsed![{source}] {error:?}",
                        html = price_html,
                        source = self.get_source(),
                        error = price_text.err(),
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                price_text.unwrap()
            };

            let available =
                product_nodes.available.is_some() && product_nodes.unavailable.is_none();
            let external_id = product_nodes.id.value().attr("href").unwrap().to_string();

            if title.is_empty() || external_id.is_empty() {
                let message = format!(
                    "Some param is invalid [{source}]: title - {title}, external_id - {id}",
                    source = self.get_source(),
                    title = title,
                    id = external_id,
                );
                sentry::capture_message(message.as_str(), sentry::Level::Warning);
                continue;
            }
            parsed_products.push(LocalParsedProduct {
                title,
                price,
                available,
                external_id,
            });
        }

        parsed_products
    }

    fn get_additional_info_url(&self, external_id: &str) -> String {
        format!("{}{}", SITE_BASE, external_id)
    }

    async fn extract_additional_info(
        &self,
        document: &Html,
        external_id: &str,
    ) -> Option<AdditionalParsedProductInfo> {
        // TODO replace tags to some standard
        let description = self.abstract_extract_description(
            &document,
            Selector::parse(".detail__tab-description").unwrap(),
            &DESCRIPTION_RE,
        );
        let available = self.abstract_parse_availability(
            document,
            Selector::parse(".btn-primary.js-buy").unwrap(),
            Selector::parse("#subscribe-product").unwrap(),
        );

        if description.is_none() || available.is_none() {
            None
        } else {
            // We should not upload images if it is not valid product
            // let image_urls = self.extract_images(document, external_id).await;
            println!(
                "characteristics {:?}",
                self.extract_characteristics(&document, external_id)
            );
            panic!("her");
            // Some(AdditionalParsedProductInfo {
            //     image_urls,
            //     description: description.unwrap(),
            //     available: available.unwrap(),
            //     characteristics: self.extract_characteristics(&document),
            // })
        }
    }
}

impl MiShopComCrawler {
    async fn extract_images(&self, document: &Html, external_id: &str) -> Vec<String> {
        let images_selector = Selector::parse(".detail-modal .detail__slides img").unwrap();
        let image_nodes = document.select(&images_selector);
        let image_urls = self.abstract_extract_image_urls(image_nodes, "data-lazy");

        self.abstract_extract_images(image_urls, external_id, SITE_BASE)
            .await
    }

    fn extract_characteristics(
        &self,
        document: &Html,
        external_id: &str,
    ) -> Vec<TypedCharacteristic> {
        let characteristic_title_selector =
            Selector::parse(".detail__table tr td.detail__table-one").unwrap();
        let characteristic_value_selector =
            Selector::parse(".detail__table tr td.detail__table-two").unwrap();
        let characteristic_title_nodes = document.select(&characteristic_title_selector);
        let characteristic_value_nodes = document.select(&characteristic_value_selector);

        let mut characteristics: Vec<TypedCharacteristic> = vec![];
        let titles = characteristic_title_nodes
            .into_iter()
            .collect::<Vec<ElementRef>>()
            .to_vec();
        let values = characteristic_value_nodes
            .into_iter()
            .collect::<Vec<ElementRef>>()
            .to_vec();

        println!("{}", external_id);
        let int_characteristics = self.extract_int_characteristics(external_id, &titles, &values);

        for int_char in int_characteristics {
            characteristics.push(TypedCharacteristic::Int(int_char));
        }

        characteristics
    }

    fn extract_int_characteristics(
        &self,
        external_id: &str,
        titles: &Vec<ElementRef>,
        values: &Vec<ElementRef>,
    ) -> Vec<IntCharacteristic> {
        let mut characteristics: Vec<IntCharacteristic> = vec![];

        for (title_index, title) in titles.into_iter().enumerate() {
            let mut value_raw = inner_text(&values.get(title_index).unwrap().inner_html());

            let clean_title = inner_text(&title.inner_html()).replace(":", "");

            let characteristic: Option<IntCharacteristic> = match clean_title.as_str() {
                "Количество ядер процессора" => {
                    int_value(&clean_title, external_id, &value_raw)
                        .map_or(None, |v| Some(IntCharacteristic::NumberOfProcessorCores(v)))
                }
                "Встроенная память (ГБ):" => {
                    int_value(&clean_title, external_id, &value_raw)
                        .map_or(None, |v| Some(IntCharacteristic::BuiltInMemory_GB(v)))
                }
                "Оперативная память (ГБ)" => {
                    int_value(&clean_title, external_id, &value_raw)
                        .map_or(None, |v| Some(IntCharacteristic::Ram_GB(v)))
                }
                "Фронтальная камера (Мп)" => {
                    int_mp_value(&clean_title, external_id, &value_raw)
                        .map_or(None, |v| Some(IntCharacteristic::FrontCamera_MP(v)))
                }
                "Разрешение видеосъемки (пикс)" => {
                    pix_int_value(&clean_title, external_id, &value_raw)
                        .map_or(None, |v| Some(IntCharacteristic::VideoResolution_Pix(v)))
                }
                "Емкость аккумулятора (мА*ч)" => {
                    int_ma_h_value(&clean_title, external_id, &value_raw)
                        .map_or(None, |v| Some(IntCharacteristic::BatteryCapacity_mA_h(v)))
                }
                "Вес (г)" => int_value(&clean_title, external_id, &value_raw)
                    .map_or(None, |v| Some(IntCharacteristic::Weight_gr(v))),
                "Версия MIUI" => int_value(&clean_title, external_id, &value_raw)
                    .map_or(None, |v| Some(IntCharacteristic::MIUIVersion(v))),
                "Версия Android" => {
                    int_android_version_value(&clean_title, external_id, &value_raw)
                        .map_or(None, |v| Some(IntCharacteristic::AndroidVersion(v)))
                }
                e => {
                    sentry::capture_message(
                        format!(
                            "Unknown characteristic ({title}) with value ({value}) for [{external_id}]",
                            title = e,
                            value = value_raw,
                            external_id = external_id,
                        )
                            .as_str(),
                        sentry::Level::Warning,
                    );
                    None
                }
            };

            if let Some(characteristic) = characteristic {
                characteristics.push(characteristic);
            }
        }

        characteristics
    }
}

fn int_android_version_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(title, external_id, &value.replace("Android", "").trim())
}

fn int_mp_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(title, external_id, &value.replace("Мп", "").trim())
}
fn int_ma_h_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(title, external_id, &value.replace("мАч", "").trim())
}

fn pix_int_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(
        title,
        external_id,
        &value.replace("K", "000").replace("К", "000"),
    )
}

fn int_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    println!("raw value {}", value);

    match i32::from_str_radix(value, 10) {
        Ok(v) => Some(v),
        Err(e) => {
            sentry::capture_message(
                format!(
                    "Can't parse characteristic ({title}) with value ({value}) for [{external_id}]: {error:?}",
                    title = title,
                    value = value,
                    external_id = external_id,
                    error = e,
                )
                .as_str(),
                sentry::Level::Warning,
            );
            None
        }
    }
}
