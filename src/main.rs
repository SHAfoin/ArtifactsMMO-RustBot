use config::Config;
use dotenv::dotenv;
use reqwest::{header::HeaderValue, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    api_url: String,
    api_token: String,
}

async fn post_request() -> Result<(), Error> {
    let url = "http://localhost:4000/tasks";
    let json_data = r#"{"title":"Problems during installation","status":"todo","priority":"medium","label":"bug"}"#;

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_static("Bearer YOUR_ACCESS_TOKEN"),
    );

    let response = client
        .post(url)
        .headers(headers)
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status Code: {}", response.status());

    let response_body = response.text().await?;

    println!("Response body: \n{}", response_body);

    Ok(())
}

// #[tokio::main]
fn main() {
    dotenv().ok();

    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .add_source(config::Environment::with_prefix("artifactsmmo"))
        .build()
        .unwrap();

    let settings: Settings = config.try_deserialize().unwrap();

    // post_request().await?;
    // Ok(())
}
