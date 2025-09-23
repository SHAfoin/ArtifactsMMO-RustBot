use anyhow::Result;
use tracing::info_span;

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
pub async fn get_characters_leaderboard(
    settings: Settings,
    name: Option<ValidatedStringWithSpaces>,
    sort: Option<Skill>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_characters_leaderboard", name = %name.as_ref().unwrap_or(&ValidatedStringWithSpaces::default()), sort = %sort.as_ref().map_or("".to_string(), |s| s.to_string()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

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
pub async fn get_account_leaderboard(
    settings: Settings,
    name: Option<ValidatedStringWithSpaces>,
    sort: Option<ScoreType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_account_leaderboard", name = %name.as_ref().unwrap_or(&ValidatedStringWithSpaces::default()), sort = %sort.as_ref().map_or("".to_string(), |s| s.to_string()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

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
