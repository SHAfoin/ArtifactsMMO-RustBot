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

/// List of all achievements.
pub async fn get_all_achievements(
    settings: Settings,
    _type: Option<AchievementType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_achievements", _type = %_type.as_ref().map_or("".to_string(), |t| t.to_string()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

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
pub async fn get_achievement(settings: Settings, code: ValidatedString) -> Result<serde_json::Value> {
    let span = info_span!("get_achievement", code = %code);
    let _enter = span.enter();

    get(settings, &format!("/achievements/{}", code), None).await
}
