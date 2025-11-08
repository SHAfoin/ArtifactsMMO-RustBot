use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::get,
    types::common::{settings::Settings, validated_string::ValidatedString},
};

/// Retrieve the details of a character.
/// https://api.artifactsmmo.com/docs/#/operations/get_character_characters__name__get
pub async fn get_character(
    settings: &Settings,
    name: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("get_character", name = %name);
    let _enter = span.enter();

    get(settings, &format!("/characters/{}", name), None).await
}
