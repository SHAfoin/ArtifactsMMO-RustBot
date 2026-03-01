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

/// List of all achievements.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_achievements_achievements_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_all_achievements(
    settings: &Settings,
    _type: Option<AchievementType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value, i64> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    get(settings, "/achievements", Some(query_params)).await
}

/// Retrieve the details of a achievement.
/// https://api.artifactsmmo.com/docs/#/operations/get_achievement_achievements__code__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_achievement(
    settings: &Settings,
    code: ValidatedString,
) -> Result<serde_json::Value, i64> {
    get(settings, &format!("/achievements/{}", code), None).await
}
