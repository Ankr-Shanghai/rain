use serde::Deserialize;
use std::fmt;

pub struct AppState {
    pub config: Config,
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "username:{} password:{} host:{} port:{} dbname:{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DbConfig,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "database: {}", self.database)
    }
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap();
        cfg.try_deserialize()
    }
}
