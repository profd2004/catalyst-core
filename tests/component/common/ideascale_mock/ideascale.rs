use std::env;

use config::{Config, ConfigError, File, FileFormat};
use dotenvy::dotenv;

#[derive(serde::Deserialize, Clone)]
pub struct IdeascaleSettings {
    pub api_url: String,
    pub api_token: String,
}

///Load ideascale configuration from file
pub fn get_configuration() -> Result<IdeascaleSettings, ConfigError> {
    dotenv().ok();
    let builder = Config::builder().add_source(File::new(
        &env::var("IDEASCALE_CONFIGURATION_FILE")
            .expect("Ideascale configuration file env variable not found"),
        FileFormat::Yaml,
    ));
    let conf = builder.build();
    match conf {
        Ok(conf) => conf.try_deserialize(),
        Err(e) => Err(e),
    }
}
