use futures::future::join_all;

use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::db::entity::category::CategorySlug;
use crate::db::entity::source::SourceName;
use crate::dto::parsed_product::LocalParsedProduct;
use crate::parse::crawler::get_crawler;
use crate::parse::layer::save::save_parsed_products;
use crate::parse::util::dedup::dedup_products;
use crate::parse::util::parse_html;
use crate::queue::postpone::postpone_page_parsing;
use crate::service::request::get_s;
use crate::ConsumerName;

pub async fn parse_category(
    source: SourceName,
    category: CategorySlug,
) -> Result<(), reqwest::Error> {
    let mut products: Vec<LocalParsedProduct> = vec![];
    let concurrent_pages = 1; // TODO move to the db settings of specific crawler

    let crawler = get_crawler(&source);

    for url in crawler.get_next_page_urls(category) {
        for page in (1..10000).step_by(concurrent_pages) {
            let mut page_requests = vec![];
            for page in page..page + concurrent_pages {
                let url = url.replace("{page}", (page).to_string().as_ref());

                page_requests.push(get_s(url));
            }

            let page_responses = join_all(page_requests).await;
            let mut all_successful = true;
            // To prevent endless requests if site is down
            let mut amount_of_fails = 0;

            let mut current_page = page;

            // TODO rewrite in stream and Mutex (daemon in then and call next request)
            for response in page_responses {
                match response {
                    Ok(response_data) => {
                        let parsed = parse_html(&response_data, crawler);
                        let mut amount_of_duplicates = 0;
                        let parsed_amount = parsed.len();

                        for parsed_product in parsed.into_iter() {
                            let will_be_duplicated = products
                                .iter()
                                .any(|p| p.external_id == parsed_product.external_id);

                            if will_be_duplicated {
                                amount_of_duplicates += 1;
                            } else {
                                products.push(parsed_product);
                            }
                        }
                        all_successful = all_successful
                            && parsed_amount != 0 // Some sites return empty page
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

                        postpone_page_parsing(
                            url.replace("{page}", (current_page).to_string().as_ref()),
                            source,
                            category,
                        )
                        .await
                        .expect("Failed to postpone page parsing");
                    }
                }

                current_page += 1;
            }

            // if last page
            if !all_successful || amount_of_fails == concurrent_pages {
                break;
            }
        }
    }
    dedup_products(&mut products, source);

    save_parsed_products(source, crawler.get_currency(), products, category).await;

    Ok(())
}
