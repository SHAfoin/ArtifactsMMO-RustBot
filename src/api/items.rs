use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string::ValidatedString,
            validated_string_with_spaces::ValidatedStringWithSpaces,
        },
        game::{item_type::ItemType, skill::Skill},
    },
};

/// Fetch items details.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_items_items_get
pub async fn get_all_items(
    settings: &Settings,
    craft_material: Option<ValidatedString>,
    craft_skill: Option<Skill>,
    max_level: Option<i64>,
    min_level: Option<i64>,
    name: Option<ValidatedStringWithSpaces>,
    _type: Option<ItemType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_items", pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()), craft_material = %craft_material.as_ref().unwrap_or(&ValidatedString::default()), craft_skill = %craft_skill.as_ref().map_or("".to_string(), |s| s.to_string()), max_level = %max_level.unwrap_or(0), min_level = %min_level.unwrap_or(0), name = %name.as_ref().unwrap_or(&ValidatedStringWithSpaces::default()), _type = %_type.as_ref().map_or("".to_string(), |t| t.as_str().to_string()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(craft_skill) = &craft_skill {
        if !craft_skill.is_crafting_skill() {
            panic!("craft_skill must be a crafting skill");
        }
    }

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
    }

    if let Some(craft_material) = craft_material {
        query_params.push(("craft_material", craft_material.to_string()));
    }

    if let Some(craft_skill) = craft_skill {
        query_params.push(("craft_skill", craft_skill.to_string()));
    }

    if let Some(max_level) = max_level {
        query_params.push(("max_level", max_level.to_string()));
    }

    if let Some(min_level) = min_level {
        query_params.push(("min_level", min_level.to_string()));
    }

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.as_str().to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/items", Some(query_params)).await
}

/// Retrieve the details of a item.
/// https://api.artifactsmmo.com/docs/#/operations/get_item_items__code__get
pub async fn get_item(settings: &Settings, code: &str) -> Result<serde_json::Value> {
    let span = info_span!("get_item", code);
    let _enter = span.enter();
    get(settings, &format!("/items/{}", code), None).await
}
