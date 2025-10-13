use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
