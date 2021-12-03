use lib::db;
use lib::db::exchange_rate::operation::get_exchange_rate_by_code;
use lib::diesel::prelude::*;
use lib::diesel::{QueryDsl, RunQueryDsl};
use lib::schema::product;
use lib::schema::product::dsl::{enabled, id};
use crate::db::historical_price::get_historical_prices;

use crate::db::product::entity::Product;
use crate::db::product_characteristic::get_all_characteristics_of_product;
use crate::dto::product::ProductInfo;
use crate::endpoint::product::ProductParams;
use crate::util::product::convert_product_prices;

pub fn get_product_info(params: &ProductParams) -> Option<ProductInfo> {
    let connection = &db::establish_connection();

    let targets = product::table.filter(id.eq(params.id).and(enabled.eq(true)));

    let product: Option<Product> = targets
        .load::<Product>(connection)
        .expect("Error loading source products")
        .into_iter()
        .next();

    product.map(|mut p| {
        let rate = get_exchange_rate_by_code(params.currency);
        // TODO no mutation by reference
        convert_product_prices(&mut p, rate);
        let characteristics = get_all_characteristics_of_product(p.id);
        // TODO remove duplicates (which come because of different external ids)
        let price_history = get_historical_prices(p.id);

        ProductInfo {
            id: p.id,
            title: p.title,
            description: p.description,
            lowest_price: p.lowest_price,
            highest_price: p.highest_price,
            images: p.images,
            category: p.category,
            characteristics,
            price_history,
        }
    })
}
