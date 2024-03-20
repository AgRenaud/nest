use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};
use toml::de::Error;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub persistence: PersistenceSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: usize,
}

#[derive(Deserialize)]
pub struct PersistenceSettings {
    pub object_storage: ObjectStorageSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ObjectStorageSettings {
    pub path: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
    #[serde(default)]
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .username(&self.username)
            .password(&self.password)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.name)
            .log_statements(log::LevelFilter::Trace)
    }
}

pub fn get_settings(path: Option<String>) -> Result<Settings, Error> {
    let path = path.unwrap_or(String::from("config.default.toml"));

    let config = std::fs::read_to_string(path).unwrap();

    toml::from_str::<Settings>(config.as_str())
}
