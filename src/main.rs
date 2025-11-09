#![allow(dead_code)]

use anyhow::Result;

use crate::{
    api::characters::get_character,
    gameplay::gathering::collect_ressource,
    types::{
        common::{settings::Settings, validated_string::ValidatedString},
        game::character::Character,
    },
};

pub mod api;
pub mod config;
pub mod gameplay;
pub mod logging;
pub mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let settings: Settings = config::app_configuration();
    let _guard = logging::init_logging();

    let character: Character;

    match get_character(&settings, "Baba".into()).await {
        Ok(m) => {
            println!("Character info: {}", serde_json::to_string_pretty(&m)?);
            character = serde_json::from_value(m["data"].clone())?;
        }
        Err(e) => {
            eprintln!("Error fetching character: {}", e);
            return Err(e);
        }
    }

    let searched_resource = ValidatedString::new("ash_tree").unwrap();
    collect_ressource(&settings, &searched_resource, &character).await?;

    Ok(())
}
