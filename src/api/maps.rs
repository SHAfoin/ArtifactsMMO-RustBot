use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string::ValidatedString,
        },
        game::map::{MapContentType, MapLayerType},
    },
};

/// Fetch maps details.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_maps_maps_get
pub async fn get_all_maps(
    settings: Settings,
    content_code: Option<ValidatedString>,
    content_type: Option<MapContentType>,
    hide_blocked_maps: Option<bool>,
    layer: Option<MapLayerType>,
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

    if let Some(hide_blocked_maps) = hide_blocked_maps {
        query_params.push(("hide_blocked_maps", hide_blocked_maps.to_string()));
    }

    if let Some(layer) = layer {
        query_params.push(("layer", layer.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/maps", Some(query_params)).await
}

/// Fetch maps details.
/// https://api.artifactsmmo.com/docs/#/operations/get_layer_maps_maps__layer__get
pub async fn get_layer_map(settings: Settings, layer: MapLayerType) -> Result<serde_json::Value> {
    let span = info_span!("get_layer_map", layer = %layer.to_string());
    let _enter = span.enter();

    get(settings, &format!("/maps/{}", layer.to_string()), None).await
}

/// Retrieve the details of a map by layer and coordinates.
/// https://api.artifactsmmo.com/docs/#/operations/get_map_by_position_maps__layer___x___y__get
pub async fn get_map_by_position(
    settings: Settings,
    x: isize,
    y: isize,
    layer: MapLayerType,
) -> Result<serde_json::Value> {
    let span = info_span!("get_map", x, y);
    let _enter = span.enter();

    get(
        settings,
        &format!("/maps/{}/{}/{}", layer.to_string(), x, y),
        None,
    )
    .await
}

/// Retrieve the details of a map by its unique ID.
/// https://api.artifactsmmo.com/docs/#/operations/get_map_by_id_maps_id__map_id__get
pub async fn get_map_by_id(settings: Settings, map_id: usize) -> Result<serde_json::Value> {
    let span = info_span!("get_map_by_id", map_id);
    let _enter = span.enter();

    get(settings, &format!("/maps/id/{}", map_id), None).await
}
