use crate::entities::builders::OrderItemBuilder;
use shared::domain::value_objects::Money;
use std::ops::Mul;
use uuid::Uuid;

pub struct OrderItem {
    id: Uuid,
    order_id: Uuid,
    product_id: Uuid,
    quantity: i32,
    price: Money,
    sub_total: Money,
}

impl OrderItem {
    pub fn builder() -> OrderItemBuilder {
        OrderItemBuilder::default()
    }

    pub fn new(
        id: Uuid,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
        price: Money,
        sub_total: Money,
    ) -> Self {
        Self {
            id,
            order_id,
            product_id,
            quantity,
            price,
            sub_total,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn order_id(&self) -> Uuid {
        self.order_id
    }

    pub fn product_id(&self) -> Uuid {
        self.product_id
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn sub_total(&self) -> &Money {
        &self.sub_total
    }

    pub fn is_price_valid(&self) -> bool {
        self.price.is_greater_than_zero() && self.price.clone().mul(self.quantity) == self.sub_total
    }
}
