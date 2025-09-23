use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::common::{
        pagination_params::PaginationParams, settings::Settings, validated_string::ValidatedString,
    },
};

/// Fetch account details.
pub async fn get_account_details(settings: Settings) -> Result<serde_json::Value> {
    let span = info_span!("get_account_details");
    let _enter = span.enter();
    get(settings, "/my/details", None).await
}

/// Fetch bank details.
pub async fn get_bank_details(settings: Settings) -> Result<serde_json::Value> {
    let span = info_span!("get_bank_details");
    let _enter = span.enter();
    get(settings, "/my/bank", None).await
}

/// Fetch all items in your bank.
pub async fn get_bank_items(
    settings: Settings,
    item_code: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_bank_items", item_code = %item_code.as_ref().unwrap_or(&ValidatedString::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = item_code {
        query_params.push(("item_code", code.to_string()));
    }

    get(settings, "/my/bank/items", Some(query_params)).await
}

/// Fetch your sell orders details.
pub async fn get_my_grandexchange_sell_orders(
    settings: Settings,
    code: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_my_grandexchange_sell_orders", code = %code.as_ref().unwrap_or(&ValidatedString::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }

    get(settings, "/my/grandexchange/orders", Some(query_params)).await
}

/// Fetch your sales history of the last 7 days.
pub async fn get_my_grandexchange_sell_history(
    settings: Settings,
    code: Option<ValidatedString>,
    id: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_my_grandexchange_sell_history", code = %code.as_ref().unwrap_or(&ValidatedString::default()), id = %id.as_ref().unwrap_or(&ValidatedString::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }

    if let Some(id) = id {
        query_params.push(("id", id.to_string()));
    }

    get(settings, "/my/grandexchange/history", Some(query_params)).await
}
