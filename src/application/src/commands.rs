use uuid::Uuid;

pub struct CreateOrderCommand {
    customer_id: Uuid,
    price: f64,
    items: Vec<CreateOrderItemDto>,
}

impl CreateOrderCommand {
    pub fn new(customer_id: Uuid, price: f64, items: Vec<CreateOrderItemDto>) -> Self {
        Self {
            customer_id,
            price,
            items,
        }
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn items(&self) -> &Vec<CreateOrderItemDto> {
        &self.items
    }
}

pub struct CreateOrderItemDto {
    product_id: Uuid,
    quantity: i32,
    price: f64,
    sub_total: f64,
}

impl CreateOrderItemDto {
    pub fn new(product_id: Uuid, quantity: i32, price: f64, sub_total: f64) -> Self {
        Self {
            product_id,
            quantity,
            price,
            sub_total,
        }
    }

    pub fn product_id(&self) -> Uuid {
        self.product_id
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn sub_total(&self) -> f64 {
        self.sub_total
    }
}
