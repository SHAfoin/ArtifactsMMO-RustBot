use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::common::{pagination_params::PaginationParams, settings::Settings},
};

/// List of all badges.
pub async fn get_all_badges(
    settings: Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_badges", pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/badges", Some(query_params)).await
}

/// Retrieve the details of a badge.
pub async fn get_badge(settings: Settings, code: &str) -> Result<serde_json::Value> {
    let span = info_span!("get_badge", code = %code);
    let _enter = span.enter();

    get(settings, &format!("/badges/{}", code), None).await
}
