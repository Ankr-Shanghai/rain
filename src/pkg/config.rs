use jsonrpc_core::IoHandler;
use serde::Deserialize;
use serde_yaml;
use std::fmt;
use std::{fs::File, io::Read};

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
    pub uris: Vec<String>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "database: {} \n uris: {:?}", self.database, self.uris)
    }
}

impl Config {
    pub fn from_env() -> Result<Self, serde_yaml::Error> {
        let mut fh = File::open("config.yaml").expect("file not found");
        let mut buf = Vec::new();
        fh.read_to_end(&mut buf)
            .expect("something went wrong reading the file");
        serde_yaml::from_str(std::str::from_utf8(&buf).unwrap())
    }
}
