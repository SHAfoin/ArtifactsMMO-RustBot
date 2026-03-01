use crate::{
    api::{
        maps::get_all_maps,
        my_characters::{action_gathering, action_move},
    },
    types::{
        common::{settings::Settings, validated_string::ValidatedString},
        game::{
            character::Character,
            map::{MapContentType, MapLayerType},
        },
    },
};
use anyhow::Result;
use tracing::{error, info};

#[tracing::instrument(skip(settings, character), target = "gameplay")]
pub async fn search_and_collect_resource(
    settings: &Settings,
    ressource_code: &ValidatedString,
    character: &mut Character,
) -> Result<serde_json::Value, i64> {
    info!(target: "gameplay", "Searching for resource '{}' on the map...", ressource_code);
    let maps = get_all_maps(
        &settings,
        Some(ressource_code),
        Some(MapContentType::Resource),
        Some(true),
        Some(MapLayerType::Overworld),
        None,
    )
    .await?;

    let x = maps["data"][0]["x"].as_i64().unwrap();
    let y = maps["data"][0]["y"].as_i64().unwrap();

    info!(target: "gameplay", "Maps found at x:{} y:{}", x, y);

    if let Err(e) = action_move(&settings, character, Some(x), Some(y), None).await {
        match e {
            490 => {}
            _ => return Err(e),
        }
    }

    let action_gathering_result = action_gathering(&settings, character).await?;

    info!(target: "gameplay", "Successfully collected resource: {}", &ressource_code);
    Ok(action_gathering_result)
}

#[tracing::instrument(skip(settings, character), target = "gameplay")]
pub async fn search_and_collect_resource_loop(
    settings: &Settings,
    ressource_code: &ValidatedString,
    character: &mut Character,
) -> Result<serde_json::Value, i64> {
    info!(target: "gameplay", "Searching for resource '{}' on the map...", ressource_code);
    let maps = get_all_maps(
        &settings,
        Some(ressource_code),
        Some(MapContentType::Resource),
        Some(true),
        Some(MapLayerType::Overworld),
        None,
    )
    .await?;

    let x = maps["data"][0]["x"].as_i64().unwrap();
    let y = maps["data"][0]["y"].as_i64().unwrap();

    info!(target: "gameplay", "Maps found at x:{} y:{}", x, y);

    if let Err(e) = action_move(&settings, character, Some(x), Some(y), None).await {
        match e {
            490 => {}
            _ => return Err(e),
        }
    }

    loop {
        action_gathering(&settings, character).await?;
        info!(target: "gameplay", "Successfully collected resource: {}. Continuing...", &ressource_code);
    }
}
