use serde::Deserialize;
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
    pub address: String,
    pub user: String,
    pub password: String,
}

pub fn get_settings(path: Option<String>) -> Result<Settings, Error> {
    let path = path.unwrap_or(String::from("default-config.toml"));

    let config = std::fs::read_to_string(path).unwrap();

    toml::from_str::<Settings>(config.as_str())
}
