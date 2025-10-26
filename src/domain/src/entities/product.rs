use crate::entities::builders::ProductBuilder;
use shared::domain::value_objects::{Money, ProductId};

pub struct Product {
    id: ProductId,
    title: String,
    quantity: i32,
    price: Money,
}

impl Product {
    pub fn builder() -> ProductBuilder {
        ProductBuilder::default()
    }

    pub fn new(id: ProductId, title: String, quantity: i32, price: Money) -> Self {
        Self {
            id,
            title,
            quantity,
            price,
        }
    }

    pub fn id(&self) -> ProductId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn price(&self) -> &Money {
        &self.price
    }
}
