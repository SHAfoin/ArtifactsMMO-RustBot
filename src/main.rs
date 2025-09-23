#![allow(dead_code)]

use anyhow::Result;

use crate::{
    api::{accounts::get_account_characters, characters::get_character, my_characters::*},
    types::{common::settings::Settings, game::character::Character},
};

pub mod api;
pub mod config;
pub mod logging;
pub mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let settings: Settings = config::app_configuration();

    logging::init_logging();

    match get_character(settings, "Baba".into()).await {
        Ok(m) => {
            println!("Character info: {}", serde_json::to_string_pretty(&m)?);
            let character: Character = serde_json::from_value(m["data"].clone())?;
            println!("Character struct: {:?}", character);
        }
        Err(e) => eprintln!("Error moving action: {}", e),
    }

    Ok(())
}
