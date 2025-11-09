use anyhow::{anyhow, Result};
use reqwest::header::HeaderValue;
use secrecy::ExposeSecret;
use tracing::{error, info};

use crate::types::{common::settings::Settings, game::character::Character};

pub async fn post_action(
    settings: &Settings,
    character: &mut Character,
    path: &str,
    json: &str,
) -> Result<serde_json::Value> {
    if (character.is_on_cooldown()) {
        let time_to_wait = character.cooldown_expiration.unwrap() - chrono::Utc::now();
        println!(
            "Character is on cooldown. Waiting for {} seconds...",
            time_to_wait.num_seconds()
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(
            time_to_wait.num_milliseconds() as u64,
        ))
        .await;
    }

    match post(settings, path, json).await {
        Ok(m) => {
            character.update_from_response(&m["data"]["character"])?;
            Ok(m)
        }
        Err(e) => Err(e),
    }
}

pub async fn post(settings: &Settings, path: &str, json: &str) -> Result<serde_json::Value> {
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

    let status = response.status();
    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .unwrap_or_else(|_| serde_json::Value::String(response_text.clone()));

    if status.is_success() {
        info!("HTTP {}", status.as_str());
        Ok(response_json)
    } else {
        error!(
            "HTTP {} - {:?}",
            status.as_str(),
            response_json["error"]["message"]
                .as_str()
                .unwrap_or_default()
        );
        Err(anyhow!(status.as_u16() as i64))
    }
}

pub async fn get(
    settings: &Settings,
    path: &str,
    query_params: Option<Vec<(&str, String)>>,
) -> Result<serde_json::Value> {
    let url = format!("{}{}", settings.api_url, path);

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", settings.api_token.expose_secret())).unwrap(),
    );

    let response = client
        .get(url)
        .headers(headers)
        .query(&query_params.unwrap_or_default())
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .unwrap_or_else(|_| serde_json::Value::String(response_text.clone()));

    if status.is_success() {
        info!("HTTP {}", status.as_str());
        Ok(response_json)
    } else {
        error!(
            "HTTP {} - {:?}",
            status.as_str(),
            response_json["error"]["message"]
                .as_str()
                .unwrap_or_default()
        );

        Err(anyhow!(status.as_u16() as i64))
    }
}
