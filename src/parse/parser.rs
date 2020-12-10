use crate::parse;
use scraper::Html;
use crate::parse::parsed_product::ParsedProduct;
use crate::parse::crawler::crawler::Crawler;

pub async fn parse<T: Crawler>(crawler: &T) -> Result<String, reqwest::Error> {
    let mut all_products: Vec<ParsedProduct> = vec![];

    for page in 0..1000 {
        let data = parse::requester::get_data(crawler.get_next_page_url(page).as_str()).await?;
        println!("page {}", page + 1);
        let document = Html::parse_document(&data);
        let current_length = all_products.len();

        crawler.extract_products(document, &mut all_products);

        if all_products.len() == current_length {
            println!("last page");
            break;
        }
    }

    // for product in all_products {
    //     println!("{:#?}", product);
    // }

    Ok("her".to_string())
}