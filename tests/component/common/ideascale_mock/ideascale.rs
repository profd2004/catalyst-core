use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct IdeascaleSettings {
    pub api_url: String,
    pub api_token: String,
}

///Load ideascale configuration from file
pub fn get_configuration() -> Result<IdeascaleSettings, ConfigError> {
    let builder = Config::builder().add_source(File::new(
        "/home/stefano/work/catalyst-core/tests/component/common/ideascale_mock/ideascale_configuration",
        FileFormat::Yaml,
    ));
    let conf = builder.build();
    match conf {
        Ok(conf) => conf.try_deserialize(),
        Err(e) => Err(e),
    }
}
