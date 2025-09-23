#![allow(dead_code)]

use anyhow::Result;

use crate::{api::my_characters::*, types::common::settings::Settings};

pub mod api;
pub mod config;
pub mod logging;
pub mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let settings: Settings = config::app_configuration();

    logging::init_logging();

    // get_account_characters(settings, "shafoin".into()).await?;
    // post_request().await?;
    let _ = action_move(settings, "Baba".into(), 1, 6).await;

    Ok(())
}
