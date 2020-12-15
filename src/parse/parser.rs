use std::collections::HashMap;
use inflector::Inflector;
use scraper::Html;
use futures::join;

use crate::parse::parsed_product::ParsedProduct;
use crate::parse::crawler::crawler::Crawler;
use crate::parse::requester::get_data;
use crate::db::repository::source_product::link_to_product;
use termion::{color, style};

pub async fn parse<T: Crawler>(crawler: &T) -> Result<(), reqwest::Error> {
    let mut all_products_by_category: HashMap<String, Vec<ParsedProduct>> = HashMap::new();

    for category in crawler.get_categories() {
        let mut products: Vec<ParsedProduct> = vec![];

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
            if a.title == b.title && a.price != b.price {
                println!(
                    "{}Warning! Same title, different prices.{} Parser: {}, title: {}, price1: {}, price2: {}",
                    color::Fg(color::Yellow),
                    style::Reset,
                    crawler.get_source().to_string().to_snake_case(),
                    a.title,
                    a.price.to_string(),
                    b.price.to_string()
                );
            }

            a.title == b.title
        });

        for product in &products {
            link_to_product(product, crawler.get_source(), &category);
        }

        all_products_by_category.insert(category.to_string().to_snake_case(), products);
    }

    Ok(())
}

fn parse_html<T: Crawler>(data: String, mut products: &mut Vec<ParsedProduct>, crawler: &T) -> bool {
    let document = Html::parse_document(&data);

    crawler.extract_products(document, &mut products)
}