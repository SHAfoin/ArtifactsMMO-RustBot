use anyhow::Result;

use crate::{
    api::utils::get,
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string_with_spaces::ValidatedStringWithSpaces,
        },
        game::{score_type::ScoreType, skill::Skill},
    },
};

/// Fetch leaderboard details.
/// https://api.artifactsmmo.com/docs/#/operations/get_characters_leaderboard_leaderboard_characters_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_characters_leaderboard(
    settings: &Settings,
    name: Option<ValidatedStringWithSpaces>,
    sort: Option<Skill>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value, i64> {
    let mut query_params = Vec::new();

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(sort) = sort {
        query_params.push(("sort", sort.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/leaderboard/characters", Some(query_params)).await
}

/// Fetch leaderboard details.
/// https://api.artifactsmmo.com/docs/#/operations/get_accounts_leaderboard_leaderboard_accounts_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_account_leaderboard(
    settings: &Settings,
    name: Option<ValidatedStringWithSpaces>,
    sort: Option<ScoreType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value, i64> {
    let mut query_params = Vec::new();

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(sort) = sort {
        query_params.push(("sort", sort.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/leaderboard/accounts", Some(query_params)).await
}
