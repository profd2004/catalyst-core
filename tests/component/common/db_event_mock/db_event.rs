use config::{Config, ConfigError, File, FileFormat};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::str::FromStr;
use uuid::Uuid;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("../src/event-db/migrations/");
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<DatabaseSettings, ConfigError> {
    let builder = Config::builder().add_source(File::new(
        "/home/stefano/work/catalyst-core/tests/component/common/db_event_configuration",
        FileFormat::Yaml,
    ));
    let conf = builder.build();
    match conf {
        Ok(conf) => conf.try_deserialize(),
        Err(e) => Err(e),
    }
}

pub fn get_configuration_with_random_db_name() -> Result<DatabaseSettings, ConfigError> {
    let builder = Config::builder()
        .add_source(File::new(
            "/home/stefano/work/catalyst-core/tests/component/common/db_event_configuration",
            FileFormat::Yaml,
        ))
        .set_override("database_name", Uuid::new_v4().to_string())
        .expect("Database name key error");
    let conf = builder.build();
    match conf {
        Ok(conf) => conf.try_deserialize(),
        Err(e) => Err(e),
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

//Create and migrate new event database
pub async fn configure_new_database(config: &DatabaseSettings) -> PgPool {
    //Create
    let mut connection = PgConnection::connect(&config.connection_string_without_db_name())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    //Migrate
    embedded::migrations::runner()
        .run(
            &mut refinery::config::Config::from_str(&config.connection_string())
                .expect("Failed to parse connection string"),
        )
        .expect("Failed to migrate the database");
    connection_pool
}
