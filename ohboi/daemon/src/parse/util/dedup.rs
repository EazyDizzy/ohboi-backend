use crate::dto::parsed_product::LocalParsedProduct;
use crate::db::entity::source::SourceName;
use lib::error_reporting;
use lib::error_reporting::ReportingContext;
use crate::ConsumerName;

pub fn dedup_products(products: &mut Vec<LocalParsedProduct>, source: SourceName) {
    products.dedup_by(|a, b| {
        if a.external_id == b.external_id && (a.price - b.price).abs() > f64::EPSILON {
            let message = format!(
                "Warning! Same external_id, different prices. Parser: {source}, id: {id}, price1: {price1}, price2: {price2}",
                source = source.to_string(),
                id = a.external_id,
                price1 = a.price.to_string(),
                price2 = b.price.to_string()
            );
            error_reporting::warning(message.as_str(), &ReportingContext {
                executor: &ConsumerName::ParseCategory,
                action: "dedup_products"
            });
        }

        a.external_id == b.external_id
    });
}