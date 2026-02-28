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

fn log_gathering_details(result: &serde_json::Value) {
    let details = if result["details"].is_object() {
        &result["details"]
    } else {
        &result["data"]["details"]
    };

    info!(target: "gameplay", "Gathering result - XP: {}", details["xp"]);

    if let Some(items) = details["items"].as_array() {
        for item in items {
            info!(
                target: "gameplay",
                "Gathering item - Code: {} x{}",
                item["code"],
                item["quantity"]
            );
        }
    }
}

#[tracing::instrument(skip(settings), target = "gameplay", fields(character = %character.name))]
pub async fn collect_ressource(
    settings: &Settings,
    ressource_code: &ValidatedString,
    character: &mut Character,
) -> Result<serde_json::Value> {
    match get_all_maps(
        &settings,
        Some(ressource_code),
        Some(MapContentType::Resource),
        Some(true),
        Some(MapLayerType::Overworld),
        None,
    )
    .await
    {
        Ok(m) => {
            let x = m["data"][0]["x"].as_i64().unwrap();
            let y = m["data"][0]["y"].as_i64().unwrap();
            info!(target: "gameplay", "Maps found at x:{} y:{}", x, y);

            match action_move(&settings, character, Some(x), Some(y), None).await {
                Ok(m) => {
                    info!(target: "gameplay", "Move result: {}", m);
                    match action_gathering(&settings, character).await {
                        Ok(m) => {
                            log_gathering_details(&m);
                            return Ok(m);
                        }
                        Err(e) => {
                            error!(target: "gameplay", "Error gathering: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) if e.to_string().contains("490") => {
                    info!(target: "gameplay", "Already at location");
                    match action_gathering(&settings, character).await {
                        Ok(m) => {
                            log_gathering_details(&m);
                            return Ok(m);
                        }
                        Err(e) => {
                            error!(target: "gameplay", "Gathering failed: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    error!(target: "gameplay", "Moving action failed: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            error!(target: "gameplay", "No maps found for resource {}: {}", ressource_code, e);
            return Err(e);
        }
    }
}
