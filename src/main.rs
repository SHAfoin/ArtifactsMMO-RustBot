use std::path;

use config::Config;
use dotenv::dotenv;
use reqwest::{header::HeaderValue, Error};
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    api_url: String,
    api_token: SecretBox<String>,
}

fn app_configuration() -> Settings {
    dotenv().ok();

    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .add_source(config::Environment::with_prefix("artifactsmmo"))
        .build()
        .unwrap();

    config.try_deserialize().unwrap()
}

async fn post(settings: Settings, path: &str, json: &str) -> Result<(), Error> {
    let url = format!("{}{}", settings.api_url, path);

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", settings.api_token.expose_secret())).unwrap(),
    );

    let response = client
        .post(url)
        .headers(headers)
        .body(json.to_owned())
        .send()
        .await?;

    println!("Status Code: {}", response.status());

    let response_body = response.text().await?;

    println!("Response body: \n{}", response_body);

    Ok(())
}

async fn get(settings: Settings, path: &str) -> Result<(), Error> {
    let url = format!("{}{}", settings.api_url, path);

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", settings.api_token.expose_secret())).unwrap(),
    );

    let response = client.get(url).headers(headers).send().await?;

    println!("Status Code: {}", response.status());

    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .unwrap_or_else(|_| serde_json::Value::String(response_text.clone()));
    let response_body = serde_json::to_string_pretty(&response_json).unwrap();

    println!("Response body: \n{}", response_body);

    Ok(())
}

async fn get_server_details(settings: Settings) -> Result<(), Error> {
    get(settings, "/").await
}

async fn get_account_details(settings: Settings) -> Result<(), Error> {
    get(settings, "/my/details").await
}

async fn get_characters(settings: Settings, character: Option<&str>) -> Result<(), Error> {
    match character {
        Some(name) => get(settings, &format!("/characters/{}", name)).await,
        None => get(settings, "/my/characters").await,
    }
}

async fn get_bank_details(settings: Settings) -> Result<(), Error> {
    get(settings, "/my/bank").await
}

async fn get_bank_items(settings: Settings) -> Result<(), Error> {
    get(settings, "/my/bank/items").await
}

async fn get_my_grandexchange_sell_orders(settings: Settings) -> Result<(), Error> {
    get(settings, "/my/grandexchange/orders").await
}

async fn get_my_grandexchange_sell_history(settings: Settings) -> Result<(), Error> {
    get(settings, "/my/grandexchange/history").await
}

async fn get_grandexchange_orders(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/grandexchange/orders/{}", code.unwrap_or_default()),
    )
    .await
}

// For a specific item only, print the last 7 days of sell history
async fn get_grandexchange_sell_history(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/grandexchange/history/{}", code)).await
}

async fn get_characters_logs(settings: Settings, character: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/my/logs/{}", character.unwrap_or_default()),
    )
    .await
}

async fn get_account_details_achievements(settings: Settings, account: &str) -> Result<(), Error> {
    get(settings, &format!("/accounts/{}/achievements", account)).await
}

async fn get_account_characters(settings: Settings, account: &str) -> Result<(), Error> {
    get(settings, &format!("/accounts/{}/characters", account)).await
}

async fn get_account(settings: Settings, account: &str) -> Result<(), Error> {
    get(settings, &format!("/accounts/{}", account)).await
}

async fn get_achievements(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/achievements/{}", code.unwrap_or_default()),
    )
    .await
}

async fn get_badges(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(settings, &format!("/badges/{}", code.unwrap_or_default())).await
}

async fn get_effects(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(settings, &format!("/effects/{}", code.unwrap_or_default())).await
}

async fn get_all_events(settings: Settings) -> Result<(), Error> {
    get(settings, "/events").await
}

async fn get_active_events(settings: Settings) -> Result<(), Error> {
    get(settings, "/events/active").await
}

async fn get_items(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(settings, &format!("/items/{}", code.unwrap_or_default())).await
}

async fn get_characters_leaderboard(settings: Settings) -> Result<(), Error> {
    get(settings, "/leaderboard/characters").await
}

async fn get_account_leaderboard(settings: Settings) -> Result<(), Error> {
    get(settings, "/leaderboard/accounts").await
}

async fn get_maps(settings: Settings, coord: Option<(&str, &str)>) -> Result<(), Error> {
    match coord {
        Some((x, y)) => get(settings, &format!("/maps/{}/{}", x, y)).await,
        None => get(settings, "/maps").await,
    }
}

async fn get_monsters(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(settings, &format!("/monsters/{}", code.unwrap_or_default())).await
}

async fn get_npcs_details(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/npcs/details/{}", code.unwrap_or_default()),
    )
    .await
}

async fn get_npcs_items(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/npcs/items/{}", code.unwrap_or_default()),
    )
    .await
}

async fn get_resources(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/resources/{}", code.unwrap_or_default()),
    )
    .await
}

async fn get_tasks(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/tasks/list/{}", code.unwrap_or_default()),
    )
    .await
}

async fn get_tasks_rewards(settings: Settings, code: Option<&str>) -> Result<(), Error> {
    get(
        settings,
        &format!("/tasks/rewards/{}", code.unwrap_or_default()),
    )
    .await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings: Settings = app_configuration();
    get_account_characters(settings, "shafoin").await?;

    // post_request().await?;
    Ok(())
}
