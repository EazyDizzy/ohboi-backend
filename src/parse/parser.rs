use crate::parse;
use scraper::Html;
use crate::parse::parsed_product::ParsedProduct;
use crate::parse::crawler::crawler::Crawler;
use inflector::Inflector;
use std::collections::HashMap;

pub async fn parse<T: Crawler>(crawler: &T) -> Result<String, reqwest::Error> {
    let mut all_products_by_category: HashMap<String, Vec<ParsedProduct>> = HashMap::new();

    for category in crawler.get_categories() {
        let mut products: Vec<ParsedProduct> = vec![];

        for page in 0..1000 {
            let data = parse::requester::get_data(crawler.get_next_page_url(category, page).as_str()).await?;
            println!("category: {}| page {}", category.to_string().to_snake_case(), page + 1);
            let document = Html::parse_document(&data);
            let current_length = products.len();

            crawler.extract_products(document, &mut products);

            if products.len() == current_length {
                println!("total products amount: {}", products.len());
                break;
            }
        }
        all_products_by_category.insert(category.to_string().to_snake_case(), products);
    }

    // for product in all_products {
    //     println!("{:#?}", product);
    // }
    // println!("Parsed: {}", all_products.len());

    Ok("her".to_string())
}