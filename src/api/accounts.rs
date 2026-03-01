use anyhow::Result;

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
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_account_achievements(
    settings: &Settings,
    account: ValidatedString,
    completed: Option<bool>,
    _type: Option<AchievementType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value, i64> {
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
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_account_characters(
    settings: &Settings,
    account: ValidatedString,
) -> Result<serde_json::Value, i64> {
    get(settings, &format!("/accounts/{}/characters", account), None).await
}

/// Retrieve the details of a character.
/// https://api.artifactsmmo.com/docs/#/operations/get_account_accounts__account__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_account(
    settings: &Settings,
    account: ValidatedString,
) -> Result<serde_json::Value, i64> {
    get(settings, &format!("/accounts/{}", account), None).await
}
