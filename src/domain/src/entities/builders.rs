use crate::{Customer, Order, OrderItem, OrderStatus, Product};
use shared::domain::value_objects::{CustomerId, Money, OrderId, OrderItemId, ProductId};
use uuid::Uuid;

#[derive(Default)]
pub struct OrderBuilder {
    id: OrderId,
    customer_id: CustomerId,
    tracking_id: Uuid,
    price: Money,
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl OrderBuilder {
    pub fn id(mut self, id: OrderId) -> Self {
        self.id = id;
        self
    }

    pub fn customer_id(mut self, id: CustomerId) -> Self {
        self.customer_id = id;
        self
    }

    pub fn tracking_id(mut self, tracking_id: Uuid) -> Self {
        self.tracking_id = tracking_id;
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
    id: OrderItemId,
    order_id: OrderId,
    product_id: ProductId,
    quantity: i32,
    price: Money,
    sub_total: Money,
}

impl OrderItemBuilder {
    pub fn id(mut self, id: OrderItemId) -> Self {
        self.id = id;
        self
    }

    pub fn order_id(mut self, id: OrderId) -> Self {
        self.order_id = id;
        self
    }

    pub fn product_id(mut self, id: ProductId) -> Self {
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

#[derive(Default)]
pub struct CustomerBuilder {
    id: CustomerId,
    user_name: String,
    first_name: String,
    last_name: String,
}

impl CustomerBuilder {
    pub fn id(mut self, id: CustomerId) -> Self {
        self.id = id;
        self
    }

    pub fn user_name(mut self, user_name: String) -> Self {
        self.user_name = user_name;
        self
    }

    pub fn first_name(mut self, first_name: String) -> Self {
        self.first_name = first_name;
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.last_name = last_name;
        self
    }

    pub fn build(self) -> Customer {
        Customer::new(self.id, self.user_name, self.first_name, self.last_name)
    }
}

#[derive(Default)]
pub struct ProductBuilder {
    id: ProductId,
    title: String,
    quantity: i32,
    price: Money,
}

impl ProductBuilder {
    pub fn id(mut self, id: ProductId) -> Self {
        self.id = id;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
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

    pub fn build(self) -> Product {
        Product::new(self.id, self.title, self.quantity, self.price)
    }
}
