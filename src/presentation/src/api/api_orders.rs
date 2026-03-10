use crate::app_state::AppState;
use crate::error::ApiError;
use crate::requests::CreateOrderRequest;
use crate::responses::OrderResponse;
use crate::validation::ValidatedJson;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, post, web};
use anyhow::anyhow;
use application::commands::{CreateOrderCommand, CreateOrderItemDto};
use application::handlers::CreateOrderCommandHandler;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde_json::json;

const ORDERS: &str = "Orders";

#[tracing::instrument(skip(req))]
#[utoipa::path(
    tag = ORDERS,
    operation_id = "create_order",
    responses(
        (status = 201, description = "Place an order item", body = [OrderResponse])
    ),
    request_body = CreateOrderRequest,
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    request: ValidatedJson<CreateOrderRequest>,
) -> Result<impl Responder, ApiError> {
    let correlation_id = req
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or("unknown".to_string());

    tracing::info!(%correlation_id, "Handling order create");

    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = CreateOrderCommandHandler::new(
        state.order_repository.clone(),
        state.customer_repository.clone(),
        state.product_repository.clone(),
    );

    let order_items = payload.items.iter().map(CreateOrderItemDto::from).collect();
    let command = CreateOrderCommand::new(
        payload.customer_id,
        Decimal::from_f64(payload.price).expect("Invalid price value"),
        order_items,
    );

    let tracking_id = handler.execute(command).await?;

    Ok(HttpResponse::Created().json(json!({ "data": OrderResponse::new(tracking_id) })))
}
