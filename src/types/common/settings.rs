use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub(crate) api_url: String,
    pub(crate) api_token: SecretBox<String>,
}
