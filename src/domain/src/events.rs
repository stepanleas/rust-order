use crate::Order;
use chrono::{DateTime, Utc};

pub struct OrderCreatedEvent {
    order: Order,
    created_at: DateTime<Utc>,
}

impl OrderCreatedEvent {
    pub fn new(order: Order) -> Self {
        Self {
            order,
            created_at: Utc::now(),
        }
    }

    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

pub struct OrderPaidEvent {
    order: Order,
    created_at: DateTime<Utc>,
}

impl OrderPaidEvent {
    pub fn new(order: Order) -> Self {
        Self {
            order,
            created_at: Utc::now(),
        }
    }

    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

pub struct OrderCancelledEvent {
    order: Order,
    created_at: DateTime<Utc>,
}

impl OrderCancelledEvent {
    pub fn new(order: Order) -> Self {
        Self {
            order,
            created_at: Utc::now(),
        }
    }

    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
