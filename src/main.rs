#![allow(dead_code)]

use std::ops::Add;

use anyhow::Result;
use tracing::{error, info, span, Level};

use crate::{
    ai::init::new_ai,
    api::resources::get_resource,
    gameplay::gathering::{search_and_collect_resource, search_and_collect_resource_loop},
    types::{
        common::{settings::Settings, validated_string::ValidatedString},
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

pub mod ai;
pub mod api;
pub mod config;
pub mod gameplay;
pub mod logging;
pub mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let settings: std::sync::Arc<Settings> = std::sync::Arc::new(config::app_configuration());
    let (_guard_http, _guard_gameplay) = logging::init_logging(true);

    let character_names = vec!["Baba"];
    let mut tasks = vec![];

    for name in character_names {
        let settings_clone = std::sync::Arc::clone(&settings);
        let name_str = name.to_string();

        let task = tokio::spawn(async move {
            let character =
                Character::fetch_character(&settings_clone, &name_str.as_str().into()).await;

            let my_span =
                span!(target: "gameplay", Level::TRACE, "", "{}", character.name.as_str());
            let _enter = my_span.enter();

            let _agent = new_ai(&settings_clone, character);

            info!(target: "gameplay", "Agent {} spawned", name_str);
        });

        tasks.push(task);
    }

    // Attendre que tous les agents terminent
    for task in tasks {
        let _ = task.await;
    }

    Ok(())
}
