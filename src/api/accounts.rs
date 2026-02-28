use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string::ValidatedString,
        },
        game::achievement_type::AchievementType,
    },
};

/// Retrieve the achievements of a account.
/// https://api.artifactsmmo.com/docs/#/operations/get_account_achievements_accounts__account__achievements_get
pub async fn get_account_achievements(
    settings: &Settings,
    account: ValidatedString,
    completed: Option<bool>,
    _type: Option<AchievementType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!(target: "http", "get_account_achievements", account = %account, completed = %completed.unwrap_or(false), _type = %_type.as_ref().map_or("".to_string(), |t| t.to_string()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(completed) = completed {
        query_params.push(("completed", completed.to_string()));
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    get(
        settings,
        &format!("/accounts/{}/achievements", account),
        Some(query_params),
    )
    .await
}

/// Fetch account character lists.
/// https://api.artifactsmmo.com/docs/#/operations/get_account_characters_accounts__account__characters_get
pub async fn get_account_characters(
    settings: &Settings,
    account: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!(target: "http", "get_account_characters", account = %account);
    let _enter = span.enter();
    get(settings, &format!("/accounts/{}/characters", account), None).await
}

/// Retrieve the details of a character.
/// https://api.artifactsmmo.com/docs/#/operations/get_account_accounts__account__get
pub async fn get_account(
    settings: &Settings,
    account: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!(target: "http", "get_account", account = %account);
    let _enter = span.enter();

    get(settings, &format!("/accounts/{}", account), None).await
}
