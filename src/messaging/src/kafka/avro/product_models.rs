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
    id: String,
    product: ProductAvroModel,
    created_at: String,
}

impl ProductCreatedEventAvroModel {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn product(&self) -> &ProductAvroModel {
        &self.product
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

#[derive(Deserialize)]
pub struct ProductUpdatedEventAvroModel {
    id: String,
    product: ProductAvroModel,
    created_at: String,
}

impl ProductUpdatedEventAvroModel {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn product(&self) -> &ProductAvroModel {
        &self.product
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

#[derive(Deserialize)]
pub struct ProductDeletedEventAvroModel {
    id: String,
    product_id: String,
    created_at: String,
}

impl ProductDeletedEventAvroModel {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn product_id(&self) -> &str {
        &self.product_id
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}
