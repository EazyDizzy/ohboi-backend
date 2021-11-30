use futures::future::join_all;
use reqwest::Error;

use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::db::entity::category::CategorySlug;
use crate::db::entity::source::SourceName;
use crate::dto::parsed_product::LocalParsedProduct;
use crate::parse::crawler::get_crawler;
use crate::parse::layer::save::save_parsed_products;
use crate::parse::util::dedup::dedup_products;
use crate::parse::util::parse_html;
use crate::service::request::get_s;
use crate::ConsumerName;

pub async fn parse_category(
    source: SourceName,
    category: CategorySlug,
) -> Result<(), reqwest::Error> {
    let mut products: Vec<LocalParsedProduct> = vec![];
    let concurrent_pages = 3; // TODO move to the db settings of specific crawler

    let crawler = get_crawler(&source);

    // TODO rewrite in stream ?
    for url in crawler.get_next_page_urls(category) {
        for page in (1..10000).step_by(concurrent_pages) {
            let page_responses = get_page_responses(&url, page, page + concurrent_pages).await;
            let mut some_products_were_parsed = true;
            // To prevent endless requests if site is down
            let mut amount_of_fails = 0;

            for response in page_responses {
                match response {
                    Ok(response_data) => {
                        let parsed_products = parse_html(&response_data, crawler);
                        let mut amount_of_duplicates = 0;
                        let parsed_amount = parsed_products.len();

                        for parsed_product in parsed_products {
                            let will_be_duplicated = products
                                .iter()
                                .any(|p| p.external_id == parsed_product.external_id);

                            if will_be_duplicated {
                                amount_of_duplicates += 1;
                            } else {
                                products.push(parsed_product);
                            }
                        }
                        some_products_were_parsed = some_products_were_parsed
                            && parsed_amount != 0 // Some sites return empty page (mi)
                            && amount_of_duplicates != parsed_amount; // But some return the last page (samsung)
                    }
                    Err(e) => {
                        amount_of_fails += 1;
                        error_reporting::warning(
                            format!(
                                "Request for page failed[{source}]: {error:?}",
                                source = source,
                                error = e
                            )
                            .as_str(),
                            &ReportingContext {
                                executor: &ConsumerName::ParseCategory,
                                action: "parse_category",
                            },
                        );
                    }
                }
            }

            // if last page
            if !some_products_were_parsed || amount_of_fails == concurrent_pages {
                break;
            }
        }
    }
    dedup_products(&mut products, source);

    save_parsed_products(source, crawler.get_currency(), products, category).await;

    Ok(())
}

async fn get_page_responses(
    url: &str,
    start_page: usize,
    end_page: usize,
) -> Vec<Result<String, Error>> {
    let mut page_requests = vec![];

    for sub_page in start_page..end_page {
        let url = url.replace("{page}", (sub_page).to_string().as_ref());

        page_requests.push(get_s(url));
    }

    join_all(page_requests).await
}
