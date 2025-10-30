#![allow(dead_code)]

use anyhow::Result;

use crate::{
    api::characters::get_character,
    types::{common::settings::Settings, game::character::Character},
};

pub mod api;
pub mod config;
pub mod logging;
pub mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let settings: Settings = config::app_configuration();

    // Keep the returned WorkerGuard alive for the lifetime of `main` so the
    // non-blocking background logger has time to flush on shutdown. Dropping
    // the guard will flush and stop the background worker.
    let _guard = logging::init_logging();

    match get_character(settings, "Bobobobobobo".into()).await {
        Ok(m) => {
            println!("Character info: {}", serde_json::to_string_pretty(&m)?);
            let character: Character = serde_json::from_value(m["data"].clone())?;
            println!("Character struct: {:?}", character);
        }
        Err(e) => eprintln!("Error moving action: {}", e),
    }

    Ok(())
}
