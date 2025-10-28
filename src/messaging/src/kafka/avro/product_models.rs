use serde::Deserialize;

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
pub struct ProductUpdatedEventAvroModel {
    product: ProductAvroModel,
    created_at: String,
}

impl ProductUpdatedEventAvroModel {
    pub fn product(&self) -> &ProductAvroModel {
        &self.product
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}
