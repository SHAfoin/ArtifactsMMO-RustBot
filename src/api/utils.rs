use anyhow::{anyhow, Result};
use reqwest::header::HeaderValue;
use secrecy::ExposeSecret;
use tracing::{error, event, info, Level};

use crate::types::{common::settings::Settings, game::character::Character};

/// Special POST request for the API that checks for cooldown and updates the character's state after the action.
/// Must be used for all actions that can trigger a cooldown, otherwise the character's state won't be updated and the bot might try to perform an action while the character is on cooldown, which will result in an error.
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

/// POST request for the API with logging.
pub async fn post(settings: &Settings, path: &str, json: &str) -> Result<serde_json::Value> {
    // ========= Créer l'URL, le client HTTP, et avoir la réponse

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

    // =========
    // ========= Récupérer le status et la réponse en JSON

    let status = response.status();
    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .unwrap_or_else(|_| serde_json::Value::String(response_text.clone()));

    // =========
    // ======== Si succès yayyy, sinon log l'erreur et renvoyer le code d'erreur (la fonction d'au dessus doit gérer selon le cas)

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

/// GET request for the API with logging.
pub async fn get(
    settings: &Settings,
    path: &str,
    query_params: Option<Vec<(&str, String)>>,
) -> Result<serde_json::Value> {
    // ========= Créer l'URL, le client HTTP, et avoir la réponse

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

    // =========
    // ========= Récupérer le status et la réponse en JSON

    let status = response.status();
    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .unwrap_or_else(|_| serde_json::Value::String(response_text.clone()));

    // =========
    // ======== Si succès yayyy, sinon log l'erreur et renvoyer le code d'erreur (la fonction d'au dessus doit gérer selon le cas)

    if status.is_success() {
        info!(target: "http", "HTTP {} OK", status.as_str());
        Ok(response_json)
    } else {
        error!(
            target: "http",
            "HTTP {} - {:?}",
            status.as_str(),
            response_json["error"]["message"]
                .as_str()
                .unwrap_or_default()
        );

        Err(anyhow!(status.as_u16() as i64))
    }
}
