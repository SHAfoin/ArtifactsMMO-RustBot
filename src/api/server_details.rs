use anyhow::Result;

use crate::{api::utils::get, types::common::settings::Settings};

/// Return the status of the game server.
/// https://api.artifactsmmo.com/docs/#/operations/get_server_details__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_server_details(settings: &Settings) -> Result<serde_json::Value, i64> {
    get(settings, "/", None).await
}
