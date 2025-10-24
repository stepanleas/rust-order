use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct OrderApprovalEventPayload {
    order_id: Uuid,
    customer_id: Uuid,
    price: String,
    created_at: NaiveDateTime,
    catalog_order_status: String,
    products: [OrderApprovalEventProduct],
}

pub struct OrderApprovalEventProduct {
    id: Uuid,
    quantity: String,
}
