use anyhow::Result;
use tracing::info_span;

use crate::{api::utils::get, types::common::settings::Settings};

/// Return the status of the game server.
/// https://api.artifactsmmo.com/docs/#/operations/get_server_details__get
pub async fn get_server_details(settings: &Settings) -> Result<serde_json::Value> {
    let span = info_span!("get_server_details");
    let _enter = span.enter();
    get(settings, "/", None).await
}
