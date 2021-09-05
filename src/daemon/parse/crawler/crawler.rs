use std::sync::Mutex;

use futures::future::{err, join_all, ok};
use futures::FutureExt;
use inflector::Inflector;
use maplit::btreemap;
use regex::Regex;
use scraper::html::Select;
use scraper::{ElementRef, Html, Selector};

use crate::local_sentry::add_category_breadcrumb;
use lib::my_enum::CurrencyEnum;
use crate::daemon::db::entity::category::CategorySlug;
use crate::daemon::db::entity::source::SourceName;
use crate::daemon::dto::parsed_product::{AdditionalParsedProductInfo, LocalParsedProduct};
use crate::daemon::queue::pub_api::postpone::postpone_image_parsing;
use crate::daemon::service::cloud::pub_api::upload_image_to_cloud;
use crate::daemon::service::html_cleaner::clean_html;
use crate::SETTINGS;

#[derive(Clone)]
struct UploadImageLaterMessage(String, String, String, SourceName);


pub trait Crawler: Sync + Send {
    fn get_site_base(&self) -> String;

    fn get_source(&self) -> SourceName;

    fn get_currency(&self) -> CurrencyEnum;

    fn get_categories(&self) -> Vec<CategorySlug>;

    fn get_next_page_urls(&self, category: CategorySlug) -> Vec<String>;

    fn extract_products(&self, document: &Html) -> Vec<LocalParsedProduct>;

    fn get_additional_info_url(&self, external_id: &str) -> String;

    fn extract_additional_info(
        &self,
        document: &Html,
        external_id: &str,
    ) -> Option<AdditionalParsedProductInfo>;

    fn abstract_extract_image_urls(
        &self,
        image_nodes: Select,
        lazy_attribute: &str,
    ) -> Vec<String> {
        let mut images_urls: Vec<String> = vec![];

        for image in image_nodes {
            let url_path: String;
            let src_tag = image.value().attr("src");

            if let Some(tag) = src_tag {
                url_path = tag.to_string();
            } else if let Some(lazy_tag) = image.value().attr(lazy_attribute) {
                url_path = lazy_tag.to_string();
            } else {
                let message = format!(
                    "both src & lazy tags not found! [{source}]",
                    source = self.get_source()
                );
                sentry::capture_message(message.as_str(), sentry::Level::Warning);
                continue;
            }

            // Mi-shop sometimes provides broken src attributes which look like src="data:,"
            if is_valid_url(&url_path) {
                images_urls.push(url_path);
            }
        }

        images_urls
    }

    fn abstract_extract_description(
        &self,
        document: &Html,
        selector: Selector,
        re: &Regex,
    ) -> Option<String> {
        let description_node = document.select(&selector).next();

        if description_node.is_none() {
            let message = format!(
                "description_node not found! [{source}]",
                source = self.get_source()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            return None;
        }

        let description_html: String = description_node.unwrap().inner_html();

        let mut description_sanitized: Vec<&str> = vec![];
        let matches = re.captures_iter(description_html.as_str());

        for capture in matches {
            for text in capture.iter() {
                if let Some(text) = text {
                    description_sanitized.push(text.as_str());
                }
            }
        }

        if description_sanitized.is_empty() {
            description_sanitized.push(r"<p>");
            description_sanitized.push(description_html.trim());
            description_sanitized.push(r"<\p>");
        }

        Some(clean_html(&description_sanitized.concat()))
    }

    fn abstract_parse_availability(
        &self,
        document: &Html,
        available_selector: Selector,
        unavailable_selector: Selector,
    ) -> Option<bool> {
        let buy_button_node = document.select(&available_selector).next();
        let unavailable_button_node = document.select(&unavailable_selector).next();

        if buy_button_node.is_none() && unavailable_button_node.is_none() {
            let message = format!(
                "both buy_button_node & unavailable_button_node not found! [{source}]",
                source = self.get_source()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            return None;
        }

        Some(buy_button_node.is_some() && unavailable_button_node.is_none())
    }
}

pub async fn upload_extracted_images(
    source: SourceName,
    image_urls: Vec<String>,
    external_id: &str,
    base: &str,
) -> Vec<String> {
    add_category_breadcrumb(
        "updating product",
        btreemap! {
            "external_id" => external_id.to_string(),
            "image_urls" => format!("{:?}", &image_urls),
            "source" => source.to_string(),
        },
        [
            "consumer.",
            &SETTINGS.queue_broker.queues.parse_category.name,
        ]
        .join(""),
    );

    let mut uploaded_urls: Vec<String> = vec![];
    let mut uploads: Vec<_> = vec![];

    let upload_later = Mutex::new(vec![]);
    for image_url in image_urls {
        let file_path = [
            "product_images/",
            &source.to_string().to_snake_case(),
            &image_url,
        ]
        .concat();

        let url: String = [base, &image_url].concat();

        uploads.push(
            upload_image_to_cloud(file_path.clone(), url.clone()).then(|success| {
                if success {
                    ok(file_path)
                } else {
                    upload_later.lock().unwrap().push(UploadImageLaterMessage(
                        file_path.clone(),
                        url,
                        external_id.to_string(),
                        source,
                    ));
                    err(file_path)
                }
            }),
        );
    }

    let uploaded_images = join_all(uploads).await;

    for filename in uploaded_images {
        if filename.is_ok() {
            uploaded_urls.push(filename.unwrap());
        }
    }

    let messages = upload_later.lock().unwrap().to_vec();
    for message in messages {
        let _schedule_result =
            postpone_image_parsing(message.0, message.1, message.2, message.3).await;
    }

    uploaded_urls
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with('/') || url.starts_with("http")
}

pub fn get_html_nodes<'result>(
    selectors: &'result ProductHtmlSelectors,
    element: &'result ElementRef,
    source: SourceName,
) -> Option<ProductHtmlNodes<'result>> {
    let id_node = element.select(&selectors.id).next();
    let title_node = element.select(&selectors.title).next();
    let price_node = element.select(&selectors.price).next();
    let available_node = element.select(&selectors.available).next();
    let unavailable_node = element.select(&selectors.unavailable).next();
    let mut valid = true;

    if id_node.is_none() {
        let message = format!("id_node not found! [{source}]", source = source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if title_node.is_none() {
        let message = format!("title_node not found! [{source}]", source = source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }
    if price_node.is_none() {
        let message = format!("price_node not found! [{source}]", source = source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if available_node.is_none() && unavailable_node.is_none() {
        let message = format!(
            "both available_node & unavailable_node not found! [{source}]",
            source = source
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if valid {
        Some(ProductHtmlNodes {
            id: id_node.unwrap(),
            title: title_node.unwrap(),
            price: price_node.unwrap(),
            available: available_node,
            unavailable: unavailable_node,
        })
    } else {
        None
    }
}

pub struct ProductHtmlSelectors {
    pub id: Selector,
    pub title: Selector,
    pub price: Selector,
    pub available: Selector,
    pub unavailable: Selector,
}

pub struct ProductHtmlNodes<'a> {
    pub id: ElementRef<'a>,
    pub title: ElementRef<'a>,
    pub price: ElementRef<'a>,
    pub available: Option<ElementRef<'a>>,
    pub unavailable: Option<ElementRef<'a>>,
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use scraper::{Html, Selector};

    use lib::my_enum::CurrencyEnum;
    use crate::daemon::parse::crawler::crawler::Crawler;
    use crate::daemon::db::entity::category::CategorySlug;
    use crate::daemon::db::entity::source::SourceName;
    use crate::daemon::dto::parsed_product::{AdditionalParsedProductInfo, LocalParsedProduct};

    #[derive(Clone)]
    pub struct EmptyCrawler {}

    impl Crawler for EmptyCrawler {
        fn get_site_base(&self) -> String {
            "her".to_string()
        }

        fn get_source(&self) -> SourceName {
            SourceName::MiShopCom
        }

        fn get_currency(&self) -> CurrencyEnum {
            CurrencyEnum::RUB
        }

        fn get_categories(&self) -> Vec<CategorySlug> {
            vec![]
        }

        fn get_next_page_urls(&self, category: CategorySlug) -> Vec<String> {
            vec![]
        }

        fn extract_products(&self, document: &Html) -> Vec<LocalParsedProduct> {
            vec![]
        }

        fn get_additional_info_url(&self, external_id: &str) -> String {
            "todo".to_string()
        }

        fn extract_additional_info(
            &self,
            document: &Html,
            external_id: &str,
        ) -> Option<AdditionalParsedProductInfo> {
            None
        }
    }

    impl EmptyCrawler {
        fn extract_image_urls(&self, document: &Html, external_id: &str) -> Vec<String> {
            let images_selector = Selector::parse("img").unwrap();
            let image_nodes = document.select(&images_selector);
            self.abstract_extract_image_urls(image_nodes, "data-lazy")
        }

        fn parse_availability(&self, document: &Html) -> Option<bool> {
            let available_selector = Selector::parse("available").unwrap();
            let unavailable_selector = Selector::parse("unavailable").unwrap();

            self.abstract_parse_availability(document, available_selector, unavailable_selector)
        }

        fn extract_description(&self, document: &Html) -> Option<String> {
            let available_selector = Selector::parse("description").unwrap();
            let text_regex = Regex::new(r"(?ms)<p>.*?</p>|<h2>.*?</h2>|<ul>.*?</ul>").unwrap();

            self.abstract_extract_description(document, available_selector, &text_regex)
        }
    }

    static CRAWLER: EmptyCrawler = EmptyCrawler {};

    #[test]
    fn it_doesnt_fail_on_no_images() {
        let document = Html::parse_document("<div></div>");

        assert!(CRAWLER.extract_image_urls(&document, "her").is_empty());
    }

    #[test]
    fn it_doesnt_fail_on_no_src_tags() {
        let document = Html::parse_document(
            "<div>\
         <img not-src=\"url.jpg\">
        </div>",
        );

        assert!(CRAWLER.extract_image_urls(&document, "her").is_empty());
    }

    #[test]
    fn it_finds_src_tags() {
        let document = Html::parse_document(
            "<div>\
         <img src=\"/url.jpg\">
        </div>",
        );

        let result = CRAWLER.extract_image_urls(&document, "her");
        assert_eq!(result.len(), 1);
        assert_eq!(*result.first().unwrap(), "/url.jpg".to_string());
    }

    #[test]
    fn it_finds_lazy_tags() {
        let document = Html::parse_document(
            "<div>\
            <img src=\"/src.jpg\">
            <img data-lazy=\"/lazy.jpg\">
        </div>",
        );

        let result = CRAWLER.extract_image_urls(&document, "her");
        assert_eq!(result.len(), 2);
        assert_eq!(*result.first().unwrap(), "/src.jpg".to_string());
        assert_eq!(*result.get(1).unwrap(), "/lazy.jpg".to_string());
    }

    #[test]
    fn it_doesnt_return_invalid_urls() {
        let document = Html::parse_document(
            "<div>\
            <img src=\"/src.jpg\">
            <img data-lazy=\"data:,\">
        </div>",
        );

        let result = CRAWLER.extract_image_urls(&document, "her");
        assert_eq!(result.len(), 1);
        assert_eq!(*result.first().unwrap(), "/src.jpg".to_string());
    }

    #[test]
    fn it_doesnt_fail_when_no_tags_presented() {
        let document = Html::parse_document(
            "<div>\
        </div>",
        );

        assert_eq!(CRAWLER.parse_availability(&document), None);
    }

    #[test]
    fn it_parses_availability() {
        let document = Html::parse_document(
            "<div>\
            <available/>
        </div>",
        );

        assert_eq!(CRAWLER.parse_availability(&document), Some(true));
    }

    #[test]
    fn it_parses_unavailability() {
        let document = Html::parse_document(
            "<div>\
            <unavailable/>
        </div>",
        );

        assert_eq!(CRAWLER.parse_availability(&document), Some(false));
    }

    #[test]
    fn it_parses_as_unavailability_when_both_tags_presented() {
        let document = Html::parse_document(
            "<div>\
            <unavailable/>
            <available/>
        </div>",
        );

        assert_eq!(CRAWLER.parse_availability(&document), Some(false));
    }

    #[test]
    fn it_doesnt_fail_on_no_description() {
        let document = Html::parse_document(
            "<div>\
        </div>",
        );

        assert_eq!(CRAWLER.extract_description(&document), None);
    }

    #[test]
    fn it_extracts_one_word_description() {
        let document = Html::parse_document(
            "<div>\
            <description>
                her
            </description>
        </div>",
        );

        assert_eq!(
            CRAWLER.extract_description(&document),
            Some(r"<p>her<\p>".to_string())
        );
    }

    #[test]
    fn it_extracts_only_valid_text() {
        let document = Html::parse_document(
            r"<div>
            <description>
                <p>her<\p>
                    <ul>
                        <li>1</li>
                        <li>2</li>
                    </ul>
            </description>
        </div>",
        );

        assert_eq!(
            CRAWLER.extract_description(&document),
            Some(r"<p>her<\p></p><ul><li>1</li><li>2</li></ul>".to_string())
        );
    }
}
