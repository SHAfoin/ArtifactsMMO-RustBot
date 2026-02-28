#![allow(dead_code)]

use anyhow::Result;
use tracing::{info, info_span, span, Level};
use tracing_subscriber::filter::targets;

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
    let (_guard_http, _guard_gameplay) = logging::init_logging(true);

    let mut baba = Character::fetch_character(&settings, &"Baba".into()).await;

    let my_span = span!(target: "gameplay", Level::TRACE, "", "{}", baba.name.as_str());
    let _enter = my_span.enter();

    let searched_resource = ValidatedString::new("ash_tree").unwrap();
    collect_ressource(&settings, &searched_resource, &mut baba).await?;

    Ok(())
}
