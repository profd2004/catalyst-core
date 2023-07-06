///Generic mock database struct
use config::{Config, ConfigError, File, FileFormat};
use dotenvy::dotenv;
use sqlx::{migrate::Migrator, Connection, Executor, PgConnection, PgPool};
use std::{env, fs, path::Path, thread};
use tokio::runtime::Runtime;
use uuid::Uuid;

//ToDO fix path,
//use sqlx::migration add logs, add errors

///Load database configuration from file
pub fn load_database_configuration() -> Result<DatabaseSettings, ConfigError> {
    dotenv().ok();
    let builder = Config::builder().add_source(File::new(
        &env::var("CONFIGURATION_FILE").unwrap_or(
            "component/common/db_mock/db_configuration"
                .to_string(),
        ),
        FileFormat::Yaml,
    ));
    let conf = builder.build();
    match conf {
        Ok(conf) => conf.try_deserialize(),
        Err(e) => Err(e),
    }
}

///Load database configuration from file with a random database name
pub fn load_database_configuration_with_random_db_name(prefix: Option<String>) -> DatabaseSettings {
    let mut db_config =
        load_database_configuration().expect("Error loading database configuration");
    db_config.name = Uuid::new_v4().to_string();
    if let Some(prefix) = prefix {
        db_config.name = prefix
    }
    db_config.name += &Uuid::new_v4().to_string();
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
pub struct DbMock {
    pub connection_pool: PgPool,
    pub settings: DatabaseSettings,
    pub persist: bool,
}

impl DbMock {
    ///Create and migrate a new database using database settings
    ///and DB_MIGRATIONS_PATH env variable or ./migrations folder
    pub async fn new(settings: DatabaseSettings) -> Self {
        dotenv().ok();
        let db_name = settings.get_db_name();
        let host_url = settings.connection_string_without_db_name();
        let db_url = settings.connection_string();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                //TODO remove println add logs
                //create db
                println!(".....Starting database {}......", &db_name);
                let mut conn = PgConnection::connect(&host_url).await.unwrap();
                conn.execute(format!(r#"CREATE DATABASE "{db_name}""#).as_str())
                    .await
                    .unwrap();
                //migrate
                println!(".....Migrating database {}......", &db_name);
                println!("current dir {}",env::current_dir().unwrap().display());
                let mut conn = PgConnection::connect(&db_url).await.unwrap();
                let migrator = Migrator::new(
                    fs::canonicalize(Path::new(
                        &env::var("DB_MIGRATIONS_PATH")
                            .unwrap_or("component/common/db_mock/migrations".to_string()),
                    ))
                    .expect("Failed to canonicalize db migrations path"),
                )
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
        DbMock::new(load_database_configuration().expect("Failed to load database configuration"))
            .await
    }

    ///Create and migrate a new database using default settings and random generated database name
    pub async fn new_with_random_name(prefix: Option<String>) -> Self {
        DbMock::new(load_database_configuration_with_random_db_name(prefix)).await
    }

    ///Connect to an existing database
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

    ///Connect to the default database
    pub async fn connect_to_default() -> Self {
        DbMock::connect(
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
}

impl Drop for DbMock {
    fn drop(&mut self) {
        if !self.persist {
            let host_url = self.settings.connection_string_without_db_name();
            let db_name = self.settings.get_db_name();
            thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                    let mut conn = PgConnection::connect(&host_url).await.unwrap();
                    //terminate existing connections
                    println!(".....Dropping database {}......", &db_name);
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
    use crate::common::db_mock::{
        db_mock::{load_database_configuration, load_database_configuration_with_random_db_name},
        DbMock,
    };

    #[tokio::test]
    async fn create_and_drop_new_db() {
        let settings = load_database_configuration_with_random_db_name(None);
        let db_mock = DbMock::new(settings).await;
    }
}
