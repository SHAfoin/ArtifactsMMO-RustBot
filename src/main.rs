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
    let settings: Settings = config::app_configuration();
    let (_guard_http, _guard_gameplay) = logging::init_logging(true);

    let baba = Character::fetch_character(&settings, &"Baba".into()).await;

    let my_span = span!(target: "gameplay", Level::TRACE, "", "{}", baba.name.as_str());
    let _enter = my_span.enter();

    let baba_agent = new_ai(baba, CharacterAdditionnalInfo::new());

    // ========== TEST RECHERCHE RESSOURCE ==========

    // let searched_resource = ValidatedString::new("ash_tree").unwrap();

    // if let Err(_) = search_and_collect_resource_loop(&settings, &searched_resource, &mut baba).await
    // {
    //     error!(
    //         target: "gameplay",
    //         "Failed to collect resource '{}', stopping program.",
    //         searched_resource,
    //     );
    // }

    Ok(())
}
