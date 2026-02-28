use anyhow::Result;

use crate::{
    api::utils::get,
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string::ValidatedString,
        },
        game::skill::Skill,
    },
};

/// Fetch resources details.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_resources_resources_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_all_resources(
    settings: &Settings,
    drop: Option<ValidatedString>,
    max_level: Option<i64>,
    min_level: Option<i64>,
    skill: Option<Skill>,
    name: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let mut query_params = Vec::new();

    if let Some(skill) = &skill {
        if !skill.is_resource_skill() {
            panic!("skill must be a resource skill");
        }
    }

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
    }

    if let Some(drop) = drop {
        query_params.push(("drop", drop.to_string()));
    }

    if let Some(max_level) = max_level {
        query_params.push(("max_level", max_level.to_string()));
    }

    if let Some(min_level) = min_level {
        query_params.push(("min_level", min_level.to_string()));
    }

    if let Some(skill) = skill {
        query_params.push(("skill", skill.to_string()));
    }

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/resources", Some(query_params)).await
}

/// Retrieve the details of a resource.
/// https://api.artifactsmmo.com/docs/#/operations/get_resource_resources__code__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_resource(settings: &Settings, code: &str) -> Result<serde_json::Value> {
    get(settings, &format!("/resources/{}", code), None).await
}
