use futures::future::join_all;

use lib::db::exchange_rate::repository::try_get_exchange_rate_by_code;
use lib::my_enum::CurrencyEnum;
use lib::service::currency_converter::convert_from_with_rate;

use crate::db::entity::category::CategorySlug;
use crate::db::entity::source::SourceName;
use crate::db::repository::product::create_if_not_exists;
use crate::db::repository::source_product::link_to_product;
use crate::dto::parsed_product::{InternationalParsedProduct, LocalParsedProduct};
use crate::queue::postpone::postpone_details_parsing;
use crate::SETTINGS;

pub async fn save_parsed_products(
    source: SourceName,
    currency: CurrencyEnum,
    products: Vec<LocalParsedProduct>,
    category: CategorySlug,
) {
    let mut savings_in_progress = vec![];
    let rate = try_get_exchange_rate_by_code(currency);

    for parsed_product in products {
        savings_in_progress.push(save_parsed_product(source, parsed_product, category, rate));

        if savings_in_progress.len() == SETTINGS.database.product_save_concurrency {
            join_all(savings_in_progress).await;
            savings_in_progress = vec![];
        }
    }

    join_all(savings_in_progress).await;
}

async fn save_parsed_product(
    source: SourceName,
    parsed_product: LocalParsedProduct,
    category: CategorySlug,
    rate: f64,
) {
    let international_parsed_product = InternationalParsedProduct {
        title: parsed_product.title,
        price: convert_from_with_rate(parsed_product.price, rate),
        original_price: parsed_product.price,
        available: parsed_product.available,
        external_id: parsed_product.external_id,
    };
    let product = create_if_not_exists(&international_parsed_product, category);

    if product.description.is_none() || product.images.is_none() {
        postpone_details_parsing(
            international_parsed_product.external_id.clone(),
            source,
            product.id,
        )
        .await
            .expect("Can't postpone deatils parsing");
    }

    link_to_product(&product, &international_parsed_product, source);
}
