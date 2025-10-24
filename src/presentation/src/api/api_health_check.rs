use crate::responses::HealthCheckResponse;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, get};
use serde_json::json;

const HEALTH: &str = "Health";

/// Handles the health check for the application startup.
///
/// This endpoint is used to check if the application is up and running.
/// It returns a response with the current status of the application.
#[utoipa::path(
    context_path = "/api/health",
    tag = HEALTH,
    responses(
        (status = 200, description = "Display application startup status", body = [HealthCheckResponse])
    ),
)]
#[get("/startup")]
pub async fn startup(_: HttpRequest) -> actix_web::Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(json!({ "data": HealthCheckResponse::new() })))
}

/// Handles the health check for the application's live status.
///
/// This endpoint is used to check if the application is currently live and accepting requests.
/// It returns a response with the current status of the application.
#[utoipa::path(
    context_path = "/api/health",
    tag = HEALTH,
    responses(
        (status = 200, description = "Display application live status", body = [HealthCheckResponse])
    ),
)]
#[get("/live")]
pub async fn live(_: HttpRequest) -> actix_web::Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(json!({ "data": HealthCheckResponse::new() })))
}

/// Handles the health check for the application's ready status.
///
/// This endpoint is used to check if the application is currently ready to handle requests.
/// It returns a response with the current status of the application.
#[utoipa::path(
    context_path = "/api/health",
    tag = HEALTH,
    responses(
        (status = 200, description = "Display application ready status", body = [HealthCheckResponse])
    ),
)]
#[get("/ready")]
pub async fn ready(_: HttpRequest) -> actix_web::Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(json!({ "data": HealthCheckResponse::new() })))
}
