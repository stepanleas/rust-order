use crate::validation::ValidationFieldError;
use application::Settings;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

const OK_STATUS: &str = "Ok";

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationFieldError>>,
}

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AppInfoResponse {
    environment: String,
}

impl AppInfoResponse {
    pub fn new(settings: Settings) -> Self {
        Self {
            environment: settings.environment,
        }
    }
}

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: String,
}

impl HealthCheckResponse {
    pub fn new() -> Self {
        Self {
            status: OK_STATUS.to_string(),
        }
    }
}

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct OrderResponse {
    pub tracking_id: Uuid,
}

impl OrderResponse {
    pub fn new(tracking_id: Uuid) -> Self {
        Self { tracking_id }
    }
}
