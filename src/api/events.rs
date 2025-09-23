use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::{
        common::{pagination_params::PaginationParams, settings::Settings},
        game::event_type::EventType,
    },
};

/// Fetch events details.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_active_events_events_active_get
pub async fn get_all_events(
    settings: Settings,
    _type: Option<EventType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_events", _type = %_type.as_ref().map_or("".to_string(), |t| t.to_string()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    if let Some(_type) = _type {
        query_params.push(("type", _type.as_str().to_string()));
    }
    get(settings, "/events", Some(query_params)).await
}

/// Fetch active events details.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_events_events_get
pub async fn get_all_active_events(
    settings: Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_active_events", pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    get(settings, "/events/active", Some(query_params)).await
}
