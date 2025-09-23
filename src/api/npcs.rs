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
        game::npc_type::NPCType,
    },
};

/// Fetch NPCs details.
pub async fn get_all_npcs(
    settings: Settings,
    name: Option<ValidatedStringWithSpaces>,
    _type: Option<NPCType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_npcs", name = %name.as_ref().unwrap_or(&ValidatedStringWithSpaces::default()), type = %_type.as_ref().map_or("".to_string(), |t| t.to_string()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, &format!("/npcs/details"), Some(query_params)).await
}

/// Retrieve the details of a NPC.
pub async fn get_npc(
    settings: Settings,
    code: Option<ValidatedString>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_npc", code = %code.as_ref().unwrap_or(&ValidatedString::default()));
    let _enter = span.enter();

    get(
        settings,
        &format!("/npcs/details/{}", code.unwrap_or_default()),
        None,
    )
    .await
}

/// Retrieve the items list of a NPC. If the NPC has items to buy, sell or trade, they will be displayed.
pub async fn get_npc_items(
    settings: Settings,
    code: &str,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_npc_items", code);
    let _enter = span.enter();

    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    get(
        settings,
        &format!("/npcs/items/{}", code),
        Some(query_params),
    )
    .await
}

/// Retrieve the list of all NPC items.
pub async fn get_all_npcs_items(
    settings: Settings,
    code: Option<ValidatedString>,
    currency: Option<ValidatedString>,
    npc: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_npcs_items", code = %code.as_ref().unwrap_or(&ValidatedString::default()), currency = %currency.as_ref().unwrap_or(&ValidatedString::default()), npc = %npc.as_ref().unwrap_or(&ValidatedString::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }
    if let Some(currency) = currency {
        query_params.push(("currency", currency.to_string()));
    }
    if let Some(npc) = npc {
        query_params.push(("npc", npc.to_string()));
    }

    get(settings, "/npcs/items", Some(query_params)).await
}
