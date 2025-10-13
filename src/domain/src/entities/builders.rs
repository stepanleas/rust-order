use crate::{Order, OrderItem, OrderStatus};
use shared::domain::value_objects::Money;
use uuid::Uuid;

#[derive(Default)]
pub struct OrderBuilder {
    id: Uuid,
    customer_id: Uuid,
    price: Money,
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl OrderBuilder {
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn customer_id(mut self, id: Uuid) -> Self {
        self.customer_id = id;
        self
    }

    pub fn tracking_id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn price(mut self, price: Money) -> Self {
        self.price = price;
        self
    }

    pub fn items(mut self, items: Vec<OrderItem>) -> Self {
        self.items = items;
        self
    }

    pub fn status(mut self, status: OrderStatus) -> Self {
        self.status = status;
        self
    }

    pub fn build(self) -> Order {
        let mut order = Order::new(self.id, self.customer_id);
        order.set_price(self.price);
        order.set_items(self.items);

        order
    }
}

#[derive(Default)]
pub struct OrderItemBuilder {
    id: Uuid,
    order_id: Uuid,
    product_id: Uuid,
    quantity: i32,
    price: Money,
    sub_total: Money,
}

impl OrderItemBuilder {
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn order_id(mut self, id: Uuid) -> Self {
        self.order_id = id;
        self
    }

    pub fn product_id(mut self, id: Uuid) -> Self {
        self.product_id = id;
        self
    }

    pub fn quantity(mut self, quantity: i32) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn price(mut self, price: Money) -> Self {
        self.price = price;
        self
    }

    pub fn sub_total(mut self, sub_total: Money) -> Self {
        self.sub_total = sub_total;
        self
    }

    pub fn build(self) -> OrderItem {
        OrderItem::new(
            self.id,
            self.order_id,
            self.product_id,
            self.quantity,
            self.price,
            self.sub_total,
        )
    }
}
