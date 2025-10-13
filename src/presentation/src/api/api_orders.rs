use crate::error::ApiError;
use crate::requests::CreateOrderRequest;
use crate::validation::ValidatedJson;
use crate::{AppState, OrderResponse};
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use anyhow::anyhow;
use application::{CreateOrderCommand, CreateOrderCommandHandler, CreateOrderItemDto};
use serde_json::json;

const ORDERS: &str = "Orders";

#[utoipa::path(
    context_path = "/api/orders",
    tag = ORDERS,
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
    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = CreateOrderCommandHandler::new(state.order_repository.clone());

    let order_items = payload.items.iter().map(CreateOrderItemDto::from).collect();
    let command = CreateOrderCommand::new(payload.customer_id, payload.price, order_items);

    let tracking_id = handler.execute(command).await?;

    Ok(HttpResponse::Created().json(json!({ "data": OrderResponse::new(tracking_id) })))
}
