use crate::Order;
use chrono::{DateTime, Utc};

pub struct OrderCreatedEvent {
    pub order: Order,
    pub created_at: DateTime<Utc>,
}

impl OrderCreatedEvent {
    pub fn new(order: Order) -> Self {
        Self {
            order,
            created_at: Utc::now(),
        }
    }
}

pub struct OrderPaidEvent {
    pub order: Order,
    pub created_at: DateTime<Utc>,
}

impl OrderPaidEvent {
    pub fn new(order: Order) -> Self {
        Self {
            order,
            created_at: Utc::now(),
        }
    }
}

pub struct OrderCancelledEvent {
    pub order: Order,
    pub created_at: DateTime<Utc>,
}

impl OrderCancelledEvent {
    pub fn new(order: Order) -> Self {
        Self {
            order,
            created_at: Utc::now(),
        }
    }
}
