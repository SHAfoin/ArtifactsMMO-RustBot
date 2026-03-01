use anyhow::Result;

use crate::{
    api::utils::get,
    types::common::{pagination_params::PaginationParams, settings::Settings},
};

/// List of all effects. Effects are used by equipment, tools, runes, consumables and monsters. An effect is an action that produces an effect on the game.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_effects_effects_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_all_effects(
    settings: &Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value, i64> {
    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    get(settings, "/effects", Some(query_params)).await
}

/// Retrieve the details of a badge.
/// https://api.artifactsmmo.com/docs/#/operations/get_effect_effects__code__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_effect(settings: &Settings, code: &str) -> Result<serde_json::Value, i64> {
    get(settings, &format!("/effects/{}", code), None).await
}
