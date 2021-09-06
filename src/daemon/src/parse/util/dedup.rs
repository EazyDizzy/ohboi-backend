use crate::dto::parsed_product::LocalParsedProduct;
use crate::db::entity::source::SourceName;

pub fn dedup_products(products: &mut Vec<LocalParsedProduct>, source: SourceName) {
    let error_margin = f64::EPSILON;

    products.dedup_by(|a, b| {
        if a.external_id == b.external_id && (a.price - b.price).abs() > error_margin {
            let message = format!(
                "Warning! Same external_id, different prices. Parser: {source}, id: {id}, price1: {price1}, price2: {price2}",
                source = source.to_string(),
                id = a.external_id,
                price1 = a.price.to_string(),
                price2 = b.price.to_string()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
        }

        a.external_id == b.external_id
    });
}