use std::collections::HashMap;
use inflector::Inflector;
use scraper::Html;
use futures::{join};

use crate::parse::parsed_product::{ParsedProduct, AdditionalParsedProductInfo};
use crate::parse::crawler::crawler::Crawler;
use crate::parse::requester::get_data;
use crate::db::repository::source_product::link_to_product;
use termion::{color, style};
use crate::db::repository::product::{create_if_not_exists, update_details};

pub async fn parse<T: Crawler>(crawler: &T) -> Result<(), reqwest::Error> {
    let mut all_products_by_category: HashMap<String, Vec<ParsedProduct>> = HashMap::new();

    for category in crawler.get_categories() {
        let mut products: Vec<ParsedProduct> = vec![];
        let current_length = products.len();

        for url in crawler.get_next_page_urls(category) {
            for page in (1..1000).step_by(5) {
                let page1 = url.replace("{page}", (page).to_string().as_ref());
                let page2 = url.replace("{page}", (page + 1).to_string().as_ref());
                let page3 = url.replace("{page}", (page + 2).to_string().as_ref());
                let page4 = url.replace("{page}", (page + 3).to_string().as_ref());
                let page5 = url.replace("{page}", (page + 4).to_string().as_ref());

                let next_pages = join!(
                     get_data(page1.as_ref()),
                     get_data(page2.as_ref()),
                     get_data(page3.as_ref()),
                     get_data(page4.as_ref()),
                     get_data(page5.as_ref()),
                );

                let parsed1 = parse_html(next_pages.0.unwrap(), &mut products, crawler);
                let parsed2 = parse_html(next_pages.1.unwrap(), &mut products, crawler);
                let parsed3 = parse_html(next_pages.2.unwrap(), &mut products, crawler);
                let parsed4 = parse_html(next_pages.3.unwrap(), &mut products, crawler);
                let parsed5 = parse_html(next_pages.4.unwrap(), &mut products, crawler);

                if !(parsed1 && parsed2 && parsed3 && parsed4 && parsed5) {
                    break;
                }
            }
        }
        products.dedup_by(|a, b| {
            if a.external_id == b.external_id && a.price != b.price {
                println!(
                    "{}Warning! Same external_id, different prices.{} Parser: {}, id: {}, price1: {}, price2: {}",
                    color::Fg(color::Yellow),
                    style::Reset,
                    crawler.get_source().to_string().to_snake_case(),
                    a.external_id,
                    a.price.to_string(),
                    b.price.to_string()
                );
            }

            a.external_id == b.external_id
        });

        println!("{}: {}", category.to_string().to_snake_case(), products.len() - current_length);

        for parsed_product in &products {
            let product = create_if_not_exists(parsed_product, &category);

            if product.description.is_none() || product.images.is_none() {
                let details = extract_additional_info(parsed_product.external_id.to_string(), crawler).await;
                update_details(&product, &details);
            }

            link_to_product(&product, parsed_product, crawler.get_source());
        }

        all_products_by_category.insert(category.to_string().to_snake_case(), products);
    }

    Ok(())
}

fn parse_html<T: Crawler>(data: String, mut products: &mut Vec<ParsedProduct>, crawler: &T) -> bool {
    let document = Html::parse_document(&data);

    crawler.extract_products(document, &mut products)
}

async fn extract_additional_info<T: Crawler>(external_id: String, crawler: &T) -> AdditionalParsedProductInfo {
    let url = crawler.get_additional_info_url(external_id);
    let data = get_data(url.as_ref()).await;

    crawler.extract_additional_info(Html::parse_document(&data.unwrap()))
}