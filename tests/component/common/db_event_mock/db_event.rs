use config::{Config, ConfigError, File, FileFormat};
use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres};
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

///Load db event configuration from file
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

///Load db event configuration from disk with a random database name
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
    ///Return connection string to the database
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    ///Return connection string to postgres instance
    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

///Create and migrate new event database
pub async fn configure_new_database(config: &DatabaseSettings) -> PgPool {
    //create
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
    //migrate
    embedded::migrations::runner()
        .run(
            &mut refinery::config::Config::from_str(&config.connection_string())
                .expect("Failed to parse connection string"),
        )
        .expect("Failed to migrate the database");
    connection_pool
}

///Insert new event with event_id and not nullable fields in the event table
pub async fn insert_event(db_event_connection: Pool<Postgres>, event_id: i32){
    sqlx::query!(r#"INSERT INTO event (row_id, name, description, committee_size, committee_threshold) VALUES($1, 'test', 'test_description', 1, 1)"#, event_id)
        .execute(&db_event_connection)
        .await
        .expect("Failed to insert event id into event database");

}