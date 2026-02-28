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
            println!("Maps found at x:{} y:{}", x, y);

            match action_move(&settings, character, Some(x), Some(y), None).await {
                Ok(m) => {
                    println!("Move result: {}", m);
                    match action_gathering(&settings, character).await {
                        Ok(m) => {
                            println!("Gathering result: {}", m);
                            return Ok(m);
                        }
                        Err(e) => {
                            println!("Error gathering: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) if e.to_string().contains("490") => {
                    println!("Already at location");
                    match action_gathering(&settings, character).await {
                        Ok(m) => {
                            println!("Gathering result: {}", m);
                            return Ok(m);
                        }
                        Err(e) => {
                            println!("Error gathering: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error moving action: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            println!(
                "Error : no maps found for resource {} ({})",
                ressource_code, e
            );
            return Err(e);
        }
    }
}
