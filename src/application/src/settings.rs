use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub environment: String,
    pub http_url: String,
    pub service_name: String,
    pub database_url: String,
    pub kafka_host: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            environment: env::var("ENVIRONMENT").unwrap_or("development".to_string()),
            http_url: env::var("HTTP_URL").unwrap_or("127.0.0.1:8081".to_string()),
            service_name: env::var("SERVICE_NAME").unwrap_or("order_api".to_string()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost:5432/order_db".to_string()),
            kafka_host: env::var("KAFKA_HOST").unwrap_or("localhost:9092".to_string()),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
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
        match env::var("ENVIRONMENT")
            .unwrap_or("development".into())
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
