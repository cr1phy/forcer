use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub secret_key: String,    
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        let cfg = cfg.try_deserialize()?;
        Ok(cfg)
    }
}