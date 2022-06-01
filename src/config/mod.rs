use config::{ConfigError, Config, File};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Database {
  pub name: String,
  pub host: String
}

#[derive(Debug, Deserialize)]
pub struct Server {
  pub host: String,
  pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct JWT {
  pub secret: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub database: Database,
  pub server: Server,
  pub jwt: JWT
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let mut s = Config::default();
    s.merge(File::with_name("default.toml"))?;
    s.merge(File::with_name("config.toml").required(false))?;
    s.try_into()
  }
}