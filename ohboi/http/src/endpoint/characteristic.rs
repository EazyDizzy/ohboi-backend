use actix_web::HttpResponse;
use serde::Serialize;
use strum::IntoEnumIterator;

use lib::db::entity::characteristic::Characteristic;
use lib::my_enum::CharacteristicGroupSlug;
use lib::util::all_characteristics::get_all_characteristics_dto;

pub fn get_all_characteristics() -> HttpResponse {
    let characteristics = get_all_characteristics_dto();

    HttpResponse::Ok().json(AllCharacteristicsResponse {
        characteristics,
        groups: get_group_order(),
    })
}

fn get_group_order() -> Vec<CharacteristicGroupSlugOrder> {
    let mut groups_order = vec![];
    for group_slug in CharacteristicGroupSlug::iter() {
        groups_order.push(CharacteristicGroupSlugOrder {
            group_slug,
            sort_key: get_group_sort_key(group_slug),
        })
    }

    groups_order
}

fn get_group_sort_key(group_slug: CharacteristicGroupSlug) -> i16 {
    match group_slug {
        CharacteristicGroupSlug::Processor => 0,
        CharacteristicGroupSlug::Memory => 1,
        CharacteristicGroupSlug::Camera => 2,
        CharacteristicGroupSlug::Display => 3,
        CharacteristicGroupSlug::Power => 4,
        CharacteristicGroupSlug::Appearance => 5,
        CharacteristicGroupSlug::Connection => 6,
        CharacteristicGroupSlug::Sensors => 7,
        CharacteristicGroupSlug::General => 8,
    }
}

#[derive(Serialize)]
struct AllCharacteristicsResponse {
    characteristics: Vec<Characteristic>,
    groups: Vec<CharacteristicGroupSlugOrder>,
}

#[derive(Serialize)]
struct CharacteristicGroupSlugOrder {
    group_slug: CharacteristicGroupSlug,
    sort_key: i16,
}
