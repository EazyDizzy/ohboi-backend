use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

use async_trait::async_trait;
use bigdecimal::Num;
use maplit::btreemap;
use regex::{Captures, Regex};
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};
use validator::HasLen;

use crate::my_enum::CurrencyEnum;
use crate::parse::crawler::crawler::{get_html_nodes, Crawler, ProductHtmlSelectors};
use crate::parse::crawler::util::*;
use crate::parse::db::entity::category::CategorySlug;
use crate::parse::db::entity::source::SourceName;
use crate::common::dto::characteristic::enum_characteristic::{
    BatteryType, DisplayType, EnumCharacteristic, MediaFormat, SimCard, Technology,
};
use crate::common::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::common::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::common::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::parse::dto::parsed_product::{
    AdditionalParsedProductInfo, LocalParsedProduct, TypedCharacteristic,
};
use crate::parse::service::html_cleaner::inner_text;

static SITE_BASE: &str = "https://mi-shop.com";

lazy_static! {
    static ref DESCRIPTION_RE: Regex =
        Regex::new(r"(?ms)<p>.*?</p>|<h2>.*?</h2>|<ul>.*?</ul>").unwrap();
    static ref NO_DESCRIPTION_RE: Regex = Regex::new(r"(?ms)[A-Za-z./ 0-9\-+–]{2,}").unwrap();
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
            let image_urls = self.extract_images(document, external_id).await;
            let start = Instant::now();
            let characteristics = self.extract_characteristics(&document, external_id);
            let duration = start.elapsed();
            println!(
                "Time elapsed in extract_characteristics() is: {:?}",
                duration
            );
            Some(AdditionalParsedProductInfo {
                image_urls,
                description: description.unwrap(),
                available: available.unwrap(),
                characteristics,
            })
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
        let mut titles: Vec<String> = characteristic_title_nodes
            .into_iter()
            .collect::<Vec<ElementRef>>()
            .into_iter()
            .map(|title| inner_text(&title.inner_html()).replace(":", ""))
            .collect();

        let mut values: Vec<String> = characteristic_value_nodes
            .into_iter()
            .collect::<Vec<ElementRef>>()
            .into_iter()
            .map(|title| inner_text(&title.inner_html()))
            .collect();

        println!("{}", external_id);
        let (int_characteristics, mut parsed_indexes) =
            self.extract_int_characteristics(external_id, &titles, &values);
        for int_char in int_characteristics {
            characteristics.push(TypedCharacteristic::Int(int_char));
        }
        self.remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

        let (float_characteristics, mut parsed_indexes) =
            self.extract_float_characteristics(external_id, &titles, &values);
        for float_char in float_characteristics {
            characteristics.push(TypedCharacteristic::Float(float_char));
        }
        self.remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

        let (string_characteristics, mut parsed_indexes) =
            self.extract_string_characteristics(external_id, &titles, &values);
        for string_char in string_characteristics {
            characteristics.push(TypedCharacteristic::String(string_char));
        }
        self.remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

        let (enum_characteristics, mut parsed_indexes) =
            self.extract_enum_characteristics(external_id, &titles, &values);
        for string_char in enum_characteristics {
            characteristics.push(TypedCharacteristic::Enum(string_char));
        }
        self.remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

        let (technology_characteristics, mut parsed_indexes) =
            self.extract_technology_characteristics(external_id, &titles, &values);
        for string_char in technology_characteristics {
            characteristics.push(TypedCharacteristic::Enum(string_char));
        }
        self.remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

        let mut parsed_indexes = self.skip_unneeded_characteristics(&titles);
        self.remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();
            sentry::capture_message(
                format!(
                    "Unknown characteristic ({title}) with value ({value}) for [{external_id}]",
                    title = title,
                    value = value,
                    external_id = external_id,
                )
                .as_str(),
                sentry::Level::Warning,
            );
        }
        characteristics
    }

    fn remove_parsed_indexes(
        &self,
        titles: &mut Vec<String>,
        values: &mut Vec<String>,
        parsed_indexes: &mut Vec<usize>,
    ) -> () {
        parsed_indexes.sort_by(|a, b| b.cmp(a));
        for index in parsed_indexes {
            titles.remove(*index);
            values.remove(*index);
        }
    }

    fn skip_unneeded_characteristics(&self, titles: &Vec<String>) -> Vec<usize> {
        let mut parsed_indexes = vec![];
        for (title_index, title) in titles.into_iter().enumerate() {
            let skip: bool = match title.as_str() {
                "Видеозапись" => true,
                "Сенсорный дисплей" => true,
                "Примечание" => true,
                "Видеоплеер" => true,
                "Аудиоплеер" => true,
                _ => false,
            };

            if skip {
                parsed_indexes.push(title_index);
            }
        }

        parsed_indexes
    }

    fn extract_technology_characteristics(
        &self,
        external_id: &str,
        titles: &Vec<String>,
        values: &Vec<String>,
    ) -> (Vec<EnumCharacteristic>, Vec<usize>) {
        let mut characteristics: Vec<EnumCharacteristic> = vec![];
        let mut parsed_indexes = vec![];

        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();
            let characteristic: Option<Technology> = match title.as_str() {
                "NFC" => {
                    parsed_indexes.push(title_index);
                    bool_value(&title, external_id, &value).map_or(None, |v| {
                        if v {
                            Some(Technology::NFC)
                        } else {
                            None
                        }
                    })
                }
                "Автофокус" => {
                    parsed_indexes.push(title_index);
                    bool_value(&title, external_id, &value).map_or(None, |v| {
                        if v {
                            Some(Technology::Autofocus)
                        } else {
                            None
                        }
                    })
                }
                "Быстрая зарядка" => {
                    parsed_indexes.push(title_index);
                    bool_value(&title, external_id, &value).map_or(None, |v| {
                        if v {
                            Some(Technology::FastCharging)
                        } else {
                            None
                        }
                    })
                }
                "ИК-порт" => {
                    parsed_indexes.push(title_index);
                    bool_value(&title, external_id, &value).map_or(None, |v| {
                        if v {
                            Some(Technology::InfraredPort)
                        } else {
                            None
                        }
                    })
                }
                "Беспроводная зарядка" => {
                    parsed_indexes.push(title_index);
                    bool_value(&title, external_id, &value).map_or(None, |v| {
                        if v {
                            Some(Technology::WirelessCharger)
                        } else {
                            None
                        }
                    })
                }
                _ => None,
            };

            if let Some(characteristic) = characteristic {
                characteristics.push(EnumCharacteristic::TechnologySupport(characteristic));
            }
        }

        (characteristics, parsed_indexes)
    }

    fn extract_string_characteristics(
        &self,
        external_id: &str,
        titles: &Vec<String>,
        values: &Vec<String>,
    ) -> (Vec<StringCharacteristic>, Vec<usize>) {
        let mut characteristics: Vec<StringCharacteristic> = vec![];
        let mut parsed_indexes = vec![];

        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();
            let characteristic: Option<StringCharacteristic> = match title.as_str() {
                "Процессор" => Some(StringCharacteristic::Processor(string_value(&value))),
                "Модель" => Some(StringCharacteristic::Model(string_value(&value))),
                "Контрастность" => {
                    Some(StringCharacteristic::Contrast(string_value(&value)))
                }
                "Соотношение сторон" => {
                    Some(StringCharacteristic::AspectRatio(string_value(&value)))
                }
                "Разрешение дисплея" => Some(
                    StringCharacteristic::DisplayResolution(string_value(&value)),
                ),
                "Видеопроцессор" => {
                    Some(StringCharacteristic::VideoProcessor(string_value(&value)))
                }
                _ => None,
            };

            if let Some(characteristic) = characteristic {
                parsed_indexes.push(title_index);
                characteristics.push(characteristic);
            }
        }

        (characteristics, parsed_indexes)
    }

    fn extract_enum_characteristics(
        &self,
        external_id: &str,
        titles: &Vec<String>,
        values: &Vec<String>,
    ) -> (Vec<EnumCharacteristic>, Vec<usize>) {
        let mut characteristics: Vec<EnumCharacteristic> = vec![];
        let mut parsed_indexes = vec![];

        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();

            match title.as_str() {
                "SIM-карта" => {
                    multiple_parse_and_capture(title, external_id, value, enum_sim_card_value)
                        .into_iter()
                        .for_each(|v| characteristics.push(EnumCharacteristic::SimCard(v)));
                    parsed_indexes.push(title_index);
                }
                "Поддерживаемые медиа форматы" => {
                    multiple_string_media_format_value(title, external_id, value)
                        .into_iter()
                        .for_each(|v| {
                            characteristics.push(EnumCharacteristic::SupportedMediaFormat(v))
                        });
                    parsed_indexes.push(title_index);
                }
                "Интернет" => {
                    multiple_parse_and_capture(
                        title,
                        external_id,
                        value,
                        enum_internet_connection_technology_value,
                    )
                    .into_iter()
                    .for_each(|v| {
                        characteristics.push(EnumCharacteristic::InternetConnectionTechnology(v))
                    });
                    parsed_indexes.push(title_index);
                }
                "Спутниковая навигация" => {
                    multiple_parse_and_capture(
                        title,
                        external_id,
                        value,
                        enum_satellite_navigation_value,
                    )
                    .into_iter()
                    .for_each(|v| characteristics.push(EnumCharacteristic::SatelliteNavigation(v)));
                    parsed_indexes.push(title_index);
                }
                "Wi-Fi (802.11)" => {
                    multiple_parse_and_capture(
                        title,
                        external_id,
                        value,
                        enum_wifi_standard_value,
                    )
                    .into_iter()
                    .for_each(|v| characteristics.push(EnumCharacteristic::WifiStandard(v)));
                    parsed_indexes.push(title_index);
                }
                "Материал" => {
                    multiple_parse_and_capture(title, external_id, value, enum_material_value)
                        .into_iter()
                        .for_each(|v| characteristics.push(EnumCharacteristic::Material(v)));
                    parsed_indexes.push(title_index);
                }
                _ => (),
            }
        }

        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();
            let characteristic: Option<EnumCharacteristic> = match title.as_str() {
                "Тип разъема для зарядки" => parse_and_capture(
                    &title,
                    external_id,
                    &value,
                    enum_charging_connector_type_value,
                )
                .map_or(None, |v| Some(EnumCharacteristic::ChargingConnectorType(v))),
                "Слот для карты памяти" => {
                    parse_and_capture(&title, external_id, &value, enum_memory_card_slot_value)
                        .map_or(None, |v| Some(EnumCharacteristic::MemoryCardSlot(v)))
                }
                "Страна производитель" => {
                    parse_and_capture(&title, external_id, &value, enum_country_value)
                        .map_or(None, |v| Some(EnumCharacteristic::ProducingCountry(v)))
                }
                "Аудиоразъем" | "Вход аудио" => {
                    if let Some(value) = NO_DESCRIPTION_RE.captures_iter(value).next() {
                        parse_and_capture(
                            &title,
                            external_id,
                            &value.get(0).unwrap().as_str(),
                            enum_audio_jack_value,
                        )
                        .map_or(None, |v| Some(EnumCharacteristic::AudioJack(v)))
                    } else {
                        None
                    }
                }
                "Аккумулятор" => {
                    parse_and_capture(&title, external_id, &value, enum_battery_type_value)
                        .map_or(None, |v| Some(EnumCharacteristic::BatteryType(v)))
                }
                "Тип дисплея" => {
                    parse_and_capture(&title, external_id, &value, enum_display_type_value)
                        .map_or(None, |v| Some(EnumCharacteristic::DisplayType(v)))
                }
                _ => None,
            };

            if let Some(characteristic) = characteristic {
                parsed_indexes.push(title_index);
                characteristics.push(characteristic);
            }
        }

        (characteristics, parsed_indexes)
    }

    fn extract_float_characteristics(
        &self,
        external_id: &str,
        titles: &Vec<String>,
        values: &Vec<String>,
    ) -> (Vec<FloatCharacteristic>, Vec<usize>) {
        let mut characteristics: Vec<FloatCharacteristic> = vec![];
        let mut parsed_indexes = vec![];

        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();

            let characteristic: Option<FloatCharacteristic> = match title.as_str() {
                "Толщина (мм)" => float_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::Thickness_mm(v))),
                "Апертура" => float_aperture_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::Aperture(v))),
                "Ширина (мм)" => float_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::Width_mm(v))),
                "Высота (мм)" => float_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::Height_mm(v))),
                "Диагональ экрана" => {
                    float_diagonal_value(&title, external_id, &value)
                        .map_or(None, |v| Some(FloatCharacteristic::ScreenDiagonal(v)))
                }
                "Bluetooth" => float_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::BluetoothVersion(v))),
                "Частота" => float_ghz_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::CPUFrequency_Ghz(v))),
                "Вес (г)" => float_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::Weight_gr(v))),
                "Версия MIUI" => float_miui_version_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::MIUIVersion(v))),
                "Версия Android" => float_android_version_value(&title, external_id, &value)
                    .map_or(None, |v| Some(FloatCharacteristic::AndroidVersion(v))),
                _ => None,
            };

            if let Some(characteristic) = characteristic {
                parsed_indexes.push(title_index);
                characteristics.push(characteristic);
            }
        }

        (characteristics, parsed_indexes)
    }

    fn extract_int_characteristics(
        &self,
        external_id: &str,
        titles: &Vec<String>,
        values: &Vec<String>,
    ) -> (Vec<IntCharacteristic>, Vec<usize>) {
        let mut characteristics: Vec<IntCharacteristic> = vec![];
        let mut parsed_indexes = vec![];

        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();

            match title.as_str() {
                "Диапазоны LTE" => {
                    multiple_int_value(title, external_id, value)
                        .into_iter()
                        .for_each(|v| characteristics.push(IntCharacteristic::LTEDiapason(v)));
                    parsed_indexes.push(title_index);
                }
                "Диапазоны GSM" => {
                    multiple_int_value(title, external_id, value)
                        .into_iter()
                        .for_each(|v| characteristics.push(IntCharacteristic::GSMDiapason(v)));
                    parsed_indexes.push(title_index);
                }
                "Диапазоны UMTS" => {
                    multiple_int_value(title, external_id, value)
                        .into_iter()
                        .for_each(|v| characteristics.push(IntCharacteristic::UMTSDiapason(v)));
                    parsed_indexes.push(title_index);
                }
                _ => (),
            }
        }
        for (title_index, title) in titles.into_iter().enumerate() {
            let value = values.get(title_index).unwrap();

            let characteristic: Option<IntCharacteristic> = match title.as_str() {
                "Количество ядер процессора" => {
                    int_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::NumberOfProcessorCores(v)))
                }
                "Гарантия (мес)" => int_guarantee_value(&title, external_id, &value)
                    .map_or(None, |v| Some(IntCharacteristic::Warranty_month(v))),
                "Встроенная память (ГБ)" => {
                    int_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::BuiltInMemory_GB(v)))
                }
                "Оперативная память (ГБ)" => {
                    int_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::Ram_GB(v)))
                }
                "Фронтальная камера (Мп)" => {
                    int_mp_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::FrontCamera_MP(v)))
                }
                "Разрешение видеосъемки (пикс)" => {
                    pix_int_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::VideoResolution_Pix(v)))
                }
                "Емкость аккумулятора (мА*ч)" => {
                    int_ma_h_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::BatteryCapacity_mA_h(v)))
                }
                "Кол-во SIM-карт" => int_value(&title, external_id, &value)
                    .map_or(None, |v| Some(IntCharacteristic::AmountOfSimCards(v))),
                "Частота кадров видеосъемки" => {
                    int_fps_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::Fps(v)))
                }
                "Плотность пикселей (PPI)" => {
                    int_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::PPI(v)))
                }
                "Максимальный объем карты памяти" => {
                    int_memory_value(&title, external_id, &value)
                        .map_or(None, |v| Some(IntCharacteristic::MaxMemoryCardSize_GB(v)))
                }
                "Яркость (кд/м²)" => int_nit_value(&title, external_id, &value)
                    .map_or(None, |v| Some(IntCharacteristic::Brightness_cd_m2(v))),
                "Частота обновления" => int_hz_value(&title, external_id, &value)
                    .map_or(None, |v| Some(IntCharacteristic::UpdateFrequency_Hz(v))),
                "Фотокамера (Мп)" => int_mp_value(&title, external_id, &value)
                    .map_or(None, |v| Some(IntCharacteristic::Camera_mp(v))),
                _ => None,
            };

            if let Some(characteristic) = characteristic {
                parsed_indexes.push(title_index);
                characteristics.push(characteristic);
            }
        }

        (characteristics, parsed_indexes)
    }
}

fn multiple_string_media_format_value(
    title: &str,
    external_id: &str,
    mut value: &str,
) -> Vec<MediaFormat> {
    let mut formats: Vec<MediaFormat> = vec![];
    let mut values: Vec<&str>;
    if value.ends_with(".") {
        value = &value[0..value.len() - 1]
    }

    if value.contains(";") {
        values = value.split(";").collect();
    } else {
        values = value.split(",").collect();
    }

    let mut index_bonus = 0;
    let mut values_copy = values.clone();
    for (i, v) in values.into_iter().enumerate() {
        let mut additional_values: Vec<&str> = vec![];
        if v.contains("/") {
            additional_values = v.split("/").collect();
        }
        if v.contains(" and ") {
            additional_values = v.split("and").collect();
        }
        if v.contains(" и ") {
            additional_values = v.split("и").collect();
        }

        if !additional_values.is_empty() {
            values_copy.remove(i + index_bonus);

            for add_v in additional_values {
                index_bonus += 1;
                values_copy.insert(i + index_bonus - 1, add_v.trim());
            }

            index_bonus -= 1;
        }
    }
    values = values_copy;
    values = values
        .iter()
        .map(|v| {
            let parsed = NO_DESCRIPTION_RE.captures_iter(v).next();
            match parsed {
                None => None,
                Some(e) => Some(e.get(0).unwrap().as_str()),
            }
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect::<Vec<&str>>()
        .to_vec();

    for v in values {
        let mapped = enum_media_format_value(v);
        let exceptions = btreemap! {
            "AAC+eAAC+" => vec![MediaFormat::AAC_plus, MediaFormat::eAAC_plus],
            "H.264 HEVC" => vec![MediaFormat::H264, MediaFormat::H265],
            "ASP" => vec![],
            "/ ASP" => vec![],
            "." => vec![],
        };
        if let Some(format) = mapped {
            if !formats.contains(&format) {
                formats.push(format);
            }
        } else if let Some(exception_values) = exceptions.get(string_value(v).as_str()) {
            for exception_format in exception_values {
                if !formats.contains(&exception_format) {
                    formats.push(*exception_format);
                }
            }
        } else {
            sentry::capture_message(
                format!(
                    "Can't parse media format characteristic ({title}) with value ({value}) for [{external_id}]: Unknown value",
                    title = title,
                    value = v,
                    external_id = external_id
                )
                    .as_str(),
                sentry::Level::Warning,
            );
        }
    }

    formats
}

#[cfg(test)]
mod tests {
    use crate::parse::crawler::mi_shop_com::multiple_string_media_format_value;
    use crate::common::dto::characteristic::enum_characteristic::MediaFormat;

    #[test]
    fn it_parses_media_format() {
        assert_eq!(
            multiple_string_media_format_value(
                "_",
                "_",
                "MP4; M4V; MKV;XVID; WAV; AAC; MP3; AMR; FLAC; APE",
            ),
            vec![
                MediaFormat::MP4,
                MediaFormat::M4V,
                MediaFormat::MKV,
                MediaFormat::XVID,
                MediaFormat::WAV,
                MediaFormat::AAC,
                MediaFormat::MP3,
                MediaFormat::AMR,
                MediaFormat::FLAC,
                MediaFormat::APE,
            ]
        );

        assert_eq!(
            multiple_string_media_format_value(
                "_",
                "_",
                "MP4; M4V; MKV; XVID; WAV; AAC/AAC+/eAAC+; MP3; AMR-NB/WB; FLAC; PCM",
            ),
            vec![
                MediaFormat::MP4,
                MediaFormat::M4V,
                MediaFormat::MKV,
                MediaFormat::XVID,
                MediaFormat::WAV,
                MediaFormat::AAC,
                MediaFormat::AAC_plus,
                MediaFormat::eAAC_plus,
                MediaFormat::MP3,
                MediaFormat::AMR_NB,
                MediaFormat::WB,
                MediaFormat::FLAC,
                MediaFormat::PCM,
            ]
        );
        assert_eq!(
            multiple_string_media_format_value(
                "_",
                "_",
                "PCM, AAC / AAC+, MP3, AMR–NB and WB, Opus, PCM/WAVE",
            ),
            vec![
                MediaFormat::PCM,
                MediaFormat::AAC,
                MediaFormat::AAC_plus,
                MediaFormat::MP3,
                MediaFormat::AMR_NB,
                MediaFormat::WB,
                MediaFormat::Opus,
                MediaFormat::WAVE,
            ]
        );
        assert_eq!(
            multiple_string_media_format_value("_", "_", "PCM, PCM/WAVE"),
            vec![MediaFormat::PCM, MediaFormat::WAVE,]
        );
        assert_eq!(
            multiple_string_media_format_value("_", "_", "Поддерживает H.263, H264 (базовый профиль / основной профиль), H.264 HEVC, MPEG4 (простой профиль / ASP), XVID, ASF / WMV, 3GI, MKV / WEBM, M4V, FLV и другие видеоформаты.Поддерживает аудиоформаты, такие как AAC / AAC +, MP3, AMR - NB и WB, FLAC, MIDI / PCM / WAVE"),
            vec![MediaFormat::H263, MediaFormat::H264, MediaFormat::MPEG4, MediaFormat::XVID, MediaFormat::ASF, MediaFormat::WMV, MediaFormat::_3GI, MediaFormat::MKV, MediaFormat::WEBM, MediaFormat::M4V, MediaFormat::FLV, MediaFormat::AAC, MediaFormat::AAC_plus, MediaFormat::MP3, MediaFormat::AMR_NB, MediaFormat::WB, MediaFormat::FLAC, MediaFormat::MIDI, MediaFormat::PCM, MediaFormat::WAVE]
        );
        assert_eq!(
            multiple_string_media_format_value("_", "_", "Видео форматы: H.265 / HEVC (основной профиль), H.264 (базовый/основной/высокий), MPEG4 (обычный/ASP) и другие. Аудио форматы: PCM, AAC / AAC + / eAAC +, MP3, AMR - NB и WB, FLAC, WAV."),
            vec![MediaFormat::H265, MediaFormat::H264,MediaFormat::MPEG4,MediaFormat::PCM,MediaFormat::AAC,MediaFormat::AAC_plus,MediaFormat::eAAC_plus,MediaFormat::MP3,MediaFormat::AMR_NB,MediaFormat::WB,MediaFormat::FLAC,MediaFormat::WAV]
        );
    }
}
