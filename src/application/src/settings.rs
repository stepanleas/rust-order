use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

pub const DEFAULT_ENV_PREFIX_NAME: &str = "ORDER";

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub environment: String,
    pub http_url: String,
    pub service_name: String,
    pub database_url: String,
    pub kafka_host: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            environment: env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            http_url: "12.0.0.1:8081".into(),
            service_name: DEFAULT_ENV_PREFIX_NAME.into(),
            database_url: "postgres://postgres:postgres@localhost:5433/order_db".into(),
            kafka_host: "localhost:9092".into(),
        }
    }
}

impl Settings {
    pub fn load(self) -> Result<Self, ConfigError> {
        let env = EnvironmentKind::from_env().unwrap();
        let env_filename = format!("configuration/{}.toml", env.as_str());

        Config::builder()
            .add_source(File::with_name("configuration/development"))
            .add_source(File::with_name(&env_filename).required(false))
            .add_source(Environment::default().prefix("APP"))
            .build()?
            .try_deserialize()
    }
}

#[derive(Debug)]
pub enum EnvironmentKind {
    Development,
    Staging,
    Production,
}

impl EnvironmentKind {
    pub fn from_env() -> Result<Self, String> {
        match env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .to_lowercase()
            .as_str()
        {
            "development" => Ok(Self::Development),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            other => Err(format!("Unsupported environment: {other}")),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EnvironmentKind::Development => "development",
            EnvironmentKind::Staging => "staging",
            EnvironmentKind::Production => "production",
        }
    }
}
