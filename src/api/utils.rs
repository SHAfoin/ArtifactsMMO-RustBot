use anyhow::{anyhow, Result};
use reqwest::header::HeaderValue;
use secrecy::ExposeSecret;
use tracing::{error, info};

use crate::types::common::settings::Settings;

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
        Err(anyhow!(status.as_u16() as usize))
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

        Err(anyhow!(status.as_u16() as usize))
    }
}
