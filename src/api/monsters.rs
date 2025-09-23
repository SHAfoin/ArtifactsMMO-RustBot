use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::common::{
        pagination_params::PaginationParams, settings::Settings, validated_string::ValidatedString,
        validated_string_with_spaces::ValidatedStringWithSpaces,
    },
};

/// Fetch maps details.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_monsters_monsters_get
pub async fn get_all_monsters(
    settings: Settings,
    drop: Option<ValidatedString>,
    max_level: Option<usize>,
    min_level: Option<usize>,
    name: Option<ValidatedStringWithSpaces>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_monsters", drop = %drop.as_ref().unwrap_or(&ValidatedString::default()), max_level = %max_level.unwrap_or(0), min_level = %min_level.unwrap_or(0), name = %name.as_ref().unwrap_or(&ValidatedStringWithSpaces::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
    }

    if let Some(drop) = drop {
        query_params.push(("drop", drop.to_string()));
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

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, &format!("/monsters"), Some(query_params)).await
}

/// Retrieve the details of a monster.
/// https://api.artifactsmmo.com/docs/#/operations/get_monster_monsters__code__get
pub async fn get_monster(settings: Settings, code: &str) -> Result<serde_json::Value> {
    let span = info_span!("get_monster", code);
    let _enter = span.enter();

    get(settings, &format!("/monsters/{}", code), None).await
}
