use config::Config;
use dotenv::dotenv;

use crate::types::common::settings::Settings;

pub fn app_configuration() -> Settings {
    dotenv().ok();

    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .add_source(config::Environment::with_prefix("artifactsmmo"))
        .build()
        .unwrap();

    config.try_deserialize().unwrap()
}
