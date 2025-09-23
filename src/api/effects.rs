use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::common::{pagination_params::PaginationParams, settings::Settings},
};

/// List of all effects. Effects are used by equipment, tools, runes, consumables and monsters. An effect is an action that produces an effect on the game.
pub async fn get_all_effects(
    settings: Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_effects", pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    get(settings, "/effects", Some(query_params)).await
}

/// Retrieve the details of a badge.
pub async fn get_effect(settings: Settings, code: &str) -> Result<serde_json::Value> {
    let span = info_span!("get_effect", code = %code);
    let _enter = span.enter();

    get(settings, &format!("/effects/{}", code), None).await
}
