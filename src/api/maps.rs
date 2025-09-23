use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string::ValidatedString,
        },
        game::map_content_type::MapContentType,
    },
};

/// Fetch maps details.
pub async fn get_all_maps(
    settings: Settings,
    content_code: Option<ValidatedString>,
    content_type: Option<MapContentType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_maps", content_code = %content_code.as_ref().unwrap_or(&ValidatedString::default()), content_type = %content_type.as_ref().map_or("".to_string(), |t| t.to_string()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(content_code) = content_code {
        query_params.push(("content_code", content_code.to_string()));
    }

    if let Some(content_type) = content_type {
        query_params.push(("content_type", content_type.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/maps", Some(query_params)).await
}

/// Retrieve the details of a map.
pub async fn get_map(settings: Settings, x: &str, y: &str) -> Result<serde_json::Value> {
    let span = info_span!("get_map", x, y);
    let _enter = span.enter();

    get(settings, &format!("/maps/{}/{}", x, y), None).await
}
