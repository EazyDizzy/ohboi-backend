use async_trait::async_trait;
use scraper::{ElementRef, Html, Selector};

use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};

#[async_trait(? Send)]
pub trait Crawler {
    fn get_source(&self) -> &SourceName;

    fn get_categories(&self) -> Vec<&CategorySlug>;

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String>;

    fn extract_products(&self, document: &Html) -> Vec<ParsedProduct>;

    fn get_additional_info_url(&self, external_id: String) -> String;

    async fn extract_additional_info(&self, document: &Html, external_id: String) -> Option<AdditionalParsedProductInfo>;
}

pub fn get_html_nodes<'a>(selectors: &'a ProductHtmlSelectors, element: &'a ElementRef, source: &'a SourceName) -> Option<ProductHtmlNodes<'a>> {
    let id_node = element.select(&selectors.id).next();
    let title_node = element.select(&selectors.title).next();
    let price_node = element.select(&selectors.price).next();
    let available_node = element.select(&selectors.available).next();
    let unavailable_node = element.select(&selectors.unavailable).next();
    let mut valid = true;

    if id_node.is_none() {
        let message = format!("id_node not found! [{}]", source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if title_node.is_none() {
        let message = format!("title_node not found! [{}]", source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }
    if price_node.is_none() {
        let message = format!("price_node not found! [{}]", source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if available_node.is_none() && unavailable_node.is_none() {
        let message = format!("both available_node & unavailable_node not found! [{}]", source);
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