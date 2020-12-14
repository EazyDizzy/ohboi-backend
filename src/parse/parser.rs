use crate::parse;
use std::collections::HashMap;
use inflector::Inflector;
use scraper::Html;
use futures::{join, FutureExt};

use crate::parse::parsed_product::ParsedProduct;
use crate::parse::crawler::crawler::Crawler;
use crate::db::repository::source_product::link_to_product;
use crate::db::entity::CategorySlug;
use std::borrow::Borrow;

pub async fn parse<T: Crawler>(crawler: &T) -> Result<String, reqwest::Error> {
    let mut all_products_by_category: HashMap<String, Vec<ParsedProduct>> = HashMap::new();

    for category in crawler.get_categories() {
        let mut products: Vec<ParsedProduct> = vec![];

        for url in crawler.get_next_page_urls(category) {
            for page in 1..1000 {
                let url_with_pagination = url.as_str().replace("{page}", (page).to_string().as_ref());
                let data = parse::requester::get_data(url_with_pagination.as_ref()).await;
                let parse_next = parse_html(data.unwrap(), &mut products, &category, crawler);

                if !parse_next {
                    break;
                }
            }
        }

        for product in &products {
            // println!("{:#?}", product);
            link_to_product(product, crawler.get_source(), &category);
        }

        all_products_by_category.insert(category.to_string().to_snake_case(), products);
    }

    Ok("her".to_string())
}

fn parse_html<T: Crawler>(data: String, mut products: &mut Vec<ParsedProduct>, category: &CategorySlug, crawler: &T) -> bool {
    let document = Html::parse_document(&data);
    let current_length = products.len();

    crawler.extract_products(document, &mut products);

    if products.len() == current_length {
        println!("{} : {}", category.to_string().to_snake_case(), products.len());
    }

    products.len() > current_length
}