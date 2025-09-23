use anyhow::Result;
use tracing::info_span;

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
async fn get_all_resources(
    settings: Settings,
    drop: Option<ValidatedString>,
    max_level: Option<usize>,
    min_level: Option<usize>,
    skill: Option<Skill>,
    name: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_resources", drop = %drop.as_ref().unwrap_or(&ValidatedString::default()), max_level = %max_level.unwrap_or(0), min_level = %min_level.unwrap_or(0), skill = %skill.as_ref().map_or("".to_string(), |s| s.to_string()), name = %name.as_ref().unwrap_or(&ValidatedString::default()), pagination = %pagination.as_ref().unwrap_or(&PaginationParams::default()));
    let _enter = span.enter();

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
async fn get_resource(settings: Settings, code: &str) -> Result<serde_json::Value> {
    let span = info_span!("get_resource", code);
    let _enter = span.enter();

    get(settings, &format!("/resources/{}", code), None).await
}
