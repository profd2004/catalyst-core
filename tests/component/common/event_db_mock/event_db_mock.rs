use config::{Config, ConfigError, File, FileFormat};
use dotenvy::dotenv;
use sqlx::{migrate::Migrator, Connection, Executor, PgConnection, PgPool};
use std::{env, path::Path, thread};
use tokio::runtime::Runtime;
use uuid::Uuid;

//ToDO fix path, copy migration from eventdb folder,
//use sqlx::migration add logs, add errors

///Load event database configuration from file
pub fn load_database_configuration() -> Result<DatabaseSettings, ConfigError> {
    dotenv().ok();
    let builder = Config::builder().add_source(File::new(
        &env::var("EVENT_DB_CONFIGURATION_FILE")
            .expect("Event db configuration file env variable not fund"),
        FileFormat::Yaml,
    ));
    let conf = builder.build();
    match conf {
        Ok(conf) => conf.try_deserialize(),
        Err(e) => Err(e),
    }
}

///Load event database configuration from file with a random database name
pub fn load_database_configuration_with_random_db_name() -> DatabaseSettings {
    let mut db_config =
        load_database_configuration().expect("Error loading event database configuration");
    db_config.name = Uuid::new_v4().to_string();
    db_config
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl DatabaseSettings {
    ///Return connection string to the database
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }

    ///Return connection string to postgres instance
    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }

    pub fn get_db_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
pub struct EventDbMock {
    pub connection_pool: PgPool,
    pub settings: DatabaseSettings,
    pub persist: bool,
}

impl EventDbMock {
    ///Create and migrate a new event database using database settings
    pub async fn new(settings: DatabaseSettings) -> Self {
        dotenv().ok();
        let db_name = settings.get_db_name();
        let host_url = settings.connection_string_without_db_name();
        let db_url = settings.connection_string();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                //TODO add logs
                println!(".....Starting db event {}......", &db_name);
                //create db
                let mut conn = PgConnection::connect(&host_url).await.unwrap();
                conn.execute(format!(r#"CREATE DATABASE "{db_name}""#).as_str())
                    .await
                    .unwrap();
                //migrate
                let mut conn = PgConnection::connect(&db_url).await.unwrap();
                let migrator = Migrator::new(Path::new(
                    &env::var("EVENT_DB_MIGRATIONS_PATH")
                        .expect("Event db migrations path env variable not fund"),
                ))
                .await
                .unwrap();
                migrator.run(&mut conn).await.expect("Migration failed");
            });
        })
        .join()
        .expect("failed to create database");

        let connection_pool = PgPool::connect(&settings.connection_string())
            .await
            .unwrap();
        Self {
            connection_pool,
            settings,
            persist: false,
        }
    }

    //This should be changed to implement Default when async trait will be implemented in rust
    ///Create and migrate a new event database using default settings from configuration file
    pub async fn new_with_default() -> Self {
        EventDbMock::new(
            load_database_configuration().expect("Failed to load event database configuration"),
        )
        .await
    }

    ///Create and migrate a new event database using default settings and random generated database name
    pub async fn new_with_random_name() -> Self {
        EventDbMock::new(load_database_configuration_with_random_db_name()).await
    }

    ///Connect to an existing event database
    pub async fn connect(settings: DatabaseSettings) -> Self {
        let connection_pool = PgPool::connect(&settings.connection_string())
            .await
            .unwrap();
        Self {
            connection_pool,
            settings,
            persist: false,
        }
    }

    ///Connect to default event database
    pub async fn connect_to_default() -> Self {
        EventDbMock::connect(
            load_database_configuration().expect("Failed to load event database configuration"),
        )
        .await
    }

    ///Get a pool to the database
    pub async fn get_pool(&self) -> PgPool {
        self.connection_pool.clone()
    }

    ///Persist the database
    pub fn persist(&mut self) {
        self.persist = true;
    }

    ///Insert new event with event_id and not nullable fields in the event table
    pub async fn insert_event(&self, event_id: i32) {
        let event_name = format!("event_test_{}", event_id);
        sqlx::query!(r#"INSERT INTO event (row_id, name, description, committee_size, committee_threshold) VALUES($1, $2, 'test_description', 1, 1)"#, event_id,event_name)
        .execute(&self.connection_pool)
        .await
        .expect("Failed to insert event id into event database");
    }

    ///Get event with event_id from event db database
    /// TODO return event struct
    pub async fn get_event(&self, event_id: i32) {
        sqlx::query!(r#"SELECT * FROM event WHERE row_id = $1"#, event_id)
            .fetch_one(&self.connection_pool)
            .await
            .expect("Failed to get event from event database");
    }
}

impl Drop for EventDbMock {
    fn drop(&mut self) {
        if !self.persist {
            let host_url = self.settings.connection_string_without_db_name();
            let db_name = self.settings.get_db_name();
            thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                    let mut conn = PgConnection::connect(&host_url).await.unwrap();
                    //terminate existing connections
                    sqlx::query(&format!(r#"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND datname = '{db_name}'"#))
                    .execute( &mut conn)
                    .await
                    .expect("Terminate all other connections");
                    conn.execute(format!(r#"DROP DATABASE "{db_name}""#).as_str())
                        .await
                        .expect("Error while querying the drop database");
                });
            })
            .join()
            .expect("failed to drop database");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::event_db_mock::{
        event_db_mock::load_database_configuration_with_random_db_name, EventDbMock,
    };

    #[tokio::test]
    async fn create_and_drop_new_db() {
        let settings = load_database_configuration_with_random_db_name();
        let event_db = EventDbMock::new(settings).await;
        event_db.insert_event(1).await;
        // get event
        let pool = event_db.get_pool().await;
        let (id, name) = sqlx::query_as::<_, (i32, String)>("SELECT row_id, name FROM event")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(id, 1);
        assert_eq!(name, "test");
    }
}
