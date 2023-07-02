use jsonrpc_core::IoHandler;
use serde::Deserialize;
use std::fmt;
pub struct AppState {
    pub config: Config,
    pub io: IoHandler,
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub path: String,
    pub dbname: String,
}

impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "path:{} dbname:{} ", self.path, self.dbname)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DbConfig,
    pub uris: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "database: {} \n uris: {}", self.database, self.uris)
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
