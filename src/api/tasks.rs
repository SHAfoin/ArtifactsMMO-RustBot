use crate::api::utils::get;
use crate::types::common::pagination_params::PaginationParams;
use crate::types::common::settings::Settings;
use crate::types::game::skill::Skill;
use crate::types::game::task_type::TaskType;
use anyhow::Result;

/// Fetch the list of all tasks.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_tasks_tasks_list_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_all_tasks(
    settings: &Settings,
    max_level: Option<i64>,
    min_level: Option<i64>,
    skill: Option<Skill>,
    _type: Option<TaskType>,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let mut query_params = Vec::new();

    if let Some(skill) = &skill {
        if !skill.is_task_skill() {
            panic!("skill must be a task skill");
        }
    }

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
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

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/tasks/list", Some(query_params)).await
}

/// Retrieve the details of a task.
/// https://api.artifactsmmo.com/docs/#/operations/get_task_tasks_list__code__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_task(settings: &Settings, code: &str) -> Result<serde_json::Value> {
    get(settings, &format!("/tasks/list/{}", code), None).await
}

/// Retrieve the details of a tasks reward.
/// https://api.artifactsmmo.com/docs/#/operations/get_tasks_reward_tasks_rewards__code__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_tasks_reward(settings: &Settings, code: &str) -> Result<serde_json::Value> {
    get(settings, &format!("/tasks/rewards/{}", code), None).await
}

/// Fetch the list of all tasks rewards. To obtain these rewards, you must exchange 6 task coins with a tasks master.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_tasks_rewards_tasks_rewards_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_all_tasks_rewards(
    settings: &Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/tasks/rewards", Some(query_params)).await
}
