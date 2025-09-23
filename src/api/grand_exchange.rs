use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::common::{
        pagination_params::PaginationParams, settings::Settings, validated_string::ValidatedString,
    },
};

/// Fetch all sell orders.
pub async fn get_all_grandexchange_orders(
    settings: Settings,
    seller: Option<ValidatedString>,
    code: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_grandexchange_orders", seller = %seller.as_ref().unwrap_or(&ValidatedString::default()), code = %code.as_ref().unwrap_or(&ValidatedString::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(seller) = seller {
        query_params.push(("seller", seller.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }

    get(settings, "/grandexchange/orders", Some(query_params)).await
}

/// Retrieve the sell order of a item.
pub async fn get_grandexchange_order(
    settings: Settings,
    id: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("get_grandexchange_order", id = %id);
    let _enter = span.enter();

    get(settings, &format!("/grandexchange/orders/{}", id), None).await
}

// For a specific item only, print the last 7 days of sell history
pub async fn get_grandexchange_sell_history(
    settings: Settings,
    code: ValidatedString,
    buyer: Option<ValidatedString>,
    seller: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_grandexchange_sell_history", code = %code, buyer = %buyer.as_ref().unwrap_or(&ValidatedString::default()), seller = %seller.as_ref().unwrap_or(&ValidatedString::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(buyer) = buyer {
        query_params.push(("buyer", buyer.to_string()));
    }

    if let Some(seller) = seller {
        query_params.push(("seller", seller.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(
        settings,
        &format!("/grandexchange/history/{}", code),
        Some(query_params),
    )
    .await
}
