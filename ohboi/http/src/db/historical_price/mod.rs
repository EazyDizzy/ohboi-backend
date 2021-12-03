use lib::db::source_product_price_history::operation::get::get_all_for;

use crate::dto::historical_price::HistoricalPrice;

pub fn get_historical_prices(product_id: i32) -> Vec<HistoricalPrice> {
    let prices = get_all_for(product_id);

    prices.into_iter().map(HistoricalPrice::from).collect()
}
