use anyhow::Result;

use crate::{
    api::utils::get,
    types::common::{settings::Settings, validated_string::ValidatedString},
};

/// Retrieve the details of a character.
/// https://api.artifactsmmo.com/docs/#/operations/get_character_characters__name__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_character(
    settings: &Settings,
    name: &ValidatedString,
) -> Result<serde_json::Value, i64> {
    get(settings, &format!("/characters/{}", name), None).await
}
