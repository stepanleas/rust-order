use crate::AppState;
use crate::error::ApiError;
use crate::general_responses::AppInfoResponse;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use anyhow::anyhow;
use serde_json::json;

const APP: &str = "App";

#[utoipa::path(
    context_path = "/api/info",
    tag = APP,
    responses(
        (status = 200, description = "Display application info", body = [AppInfoResponse])
    )
)]
#[get("")]
pub async fn info(req: HttpRequest) -> Result<impl Responder, ApiError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    Ok(HttpResponse::Ok().json(json!({ "data": AppInfoResponse::new(state.settings.clone()) })))
}
