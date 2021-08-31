use futures::future::join_all;
use maplit::btreemap;

use crate::daemon::parse::crawler::get_crawler;
use crate::daemon::db::entity::category::CategorySlug;
use crate::daemon::db::entity::source::SourceName;
use crate::daemon::dto::parsed_product::LocalParsedProduct;
use crate::daemon::queue::pub_api::postpone::postpone_page_parsing;

use crate::daemon::service::request::pub_api::get_data_s;
use crate::daemon::parse::layer::save::save_parsed_products;
use crate::daemon::parse::util::dedup::dedup_products;
use crate::daemon::parse::util::{parse_html, add_parse_breadcrumb};

pub async fn parse_category(
    source: SourceName,
    category: CategorySlug,
) -> Result<(), reqwest::Error> {
    add_parse_breadcrumb(
        "in progress",
        btreemap! {
            "crawler" => source.to_string(),
            "category" => category.to_string(),
        },
    );

    let mut products: Vec<LocalParsedProduct> = vec![];
    let concurrent_pages = 1; // TODO move to the db settings of specific crawler

    let crawler = get_crawler(&source);

    for url in crawler.get_next_page_urls(category) {
        for page in (1..10000).step_by(concurrent_pages) {
            let mut page_requests = vec![];
            for page in page..page + concurrent_pages {
                let url = url.replace("{page}", (page).to_string().as_ref());

                page_requests.push(get_data_s(url));
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

                        parsed.iter().for_each(|parsed_product| {
                            let will_be_duplicated = products
                                .iter()
                                .any(|p| p.external_id == parsed_product.external_id);

                            if will_be_duplicated {
                                amount_of_duplicates += 1;
                            } else {
                                products.push(parsed_product.clone());
                            }
                        });
                        all_successful = all_successful
                            && !parsed.is_empty() // Some sites return empty page
                            && amount_of_duplicates != parsed.len(); // But some return the last page (samsung)
                    }
                    Err(e) => {
                        amount_of_fails += 1;
                        sentry::capture_message(
                            format!(
                                "Request for page failed[{source}]: {error:?}",
                                source = source,
                                error = e
                            )
                            .as_str(),
                            sentry::Level::Warning,
                        );

                        let _result = postpone_page_parsing(
                            url.replace("{page}", (current_page).to_string().as_ref()),
                            source,
                            category,
                        )
                        .await;
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

    add_parse_breadcrumb(
        "parsed",
        btreemap! {
            "crawler" => source.to_string(),
            "category" => category.to_string(),
            "length" => products.len().to_string()
        },
    );

    save_parsed_products(source, crawler.get_currency(), products, category).await;

    Ok(())
}
