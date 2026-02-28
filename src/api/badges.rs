use anyhow::Result;

use crate::{
    api::utils::get,
    types::common::{pagination_params::PaginationParams, settings::Settings},
};

/// List of all badges.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_badges_badges_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_all_badges(
    settings: &Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/badges", Some(query_params)).await
}

/// Retrieve the details of a badge.
/// https://api.artifactsmmo.com/docs/#/operations/get_badge_badges__code__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_badge(settings: &Settings, code: &str) -> Result<serde_json::Value> {
    get(settings, &format!("/badges/{}", code), None).await
}
