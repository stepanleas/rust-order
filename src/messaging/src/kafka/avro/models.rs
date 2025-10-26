use serde::Deserialize;

#[derive(Deserialize)]
pub struct CustomerCreatedEventAvroModel {
    customer: CustomerAvroModel,
    created_at: String,
}

#[derive(Deserialize)]
pub struct CustomerAvroModel {
    id: String,
    user_name: String,
    first_name: String,
    last_name: String,
}

impl CustomerCreatedEventAvroModel {
    pub fn customer(&self) -> &CustomerAvroModel {
        &self.customer
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

impl CustomerAvroModel {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}

#[derive(Deserialize)]
pub struct ProductCreatedEventAvroModel {
    product: ProductAvroModel,
    created_at: String,
}

impl ProductCreatedEventAvroModel {
    pub fn product(&self) -> &ProductAvroModel {
        &self.product
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

#[derive(Deserialize)]
pub struct ProductAvroModel {
    id: String,
    title: String,
    quantity: i32,
    price: String,
}

impl ProductAvroModel {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn price(&self) -> &str {
        &self.price
    }
}
