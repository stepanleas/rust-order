use crate::entities::builders::OrderItemBuilder;
use shared::domain::value_objects::{Money, OrderId, OrderItemId, ProductId};
use std::ops::Mul;

pub struct OrderItem {
    id: OrderItemId,
    order_id: OrderId,
    product_id: ProductId,
    quantity: i32,
    price: Money,
    sub_total: Money,
}

impl OrderItem {
    pub fn builder() -> OrderItemBuilder {
        OrderItemBuilder::default()
    }

    pub fn new(
        id: OrderItemId,
        order_id: OrderId,
        product_id: ProductId,
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

    pub fn id(&self) -> OrderItemId {
        self.id
    }

    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn product_id(&self) -> ProductId {
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
