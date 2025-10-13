use application::CreateOrderItemDto;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[readonly::make]
#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateOrderRequest {
    pub customer_id: Uuid,

    #[validate(range(min = 1.0, message = "Price must be greater than 0"))]
    pub price: f64,

    pub items: Vec<CreateOrderItem>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateOrderItem {
    pub product_id: Uuid,

    #[validate(range(min = 0, message = "Quantity must be greater or equal to 0"))]
    pub quantity: i32,

    #[validate(range(min = 1.0, message = "Price must be greater than 0"))]
    pub price: f64,

    #[validate(range(min = 1.0, message = "Price must be greater than 0"))]
    pub sub_total: f64,
}

impl From<&CreateOrderItem> for CreateOrderItemDto {
    fn from(item: &CreateOrderItem) -> Self {
        CreateOrderItemDto::new(item.product_id, item.quantity, item.price, item.sub_total)
    }
}
