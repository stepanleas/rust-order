use domain::{Customer, Order, Product};
use uuid::Uuid;

pub trait OrderRepository {
    fn find_by_tracking_id(&self, tracking_id: Uuid) -> anyhow::Result<Order>;
    fn save(&self, entity: &Order) -> anyhow::Result<()>;
}

pub trait CustomerRepository {
    fn find_by_id(&self, customer_id: Uuid) -> anyhow::Result<Customer>;
    fn save(&self, entity: Customer) -> anyhow::Result<()>;
}

pub trait ProductRepository {
    fn find_by_id(&self, product_id: Uuid) -> anyhow::Result<Product>;
    fn save(&self, product: Product) -> anyhow::Result<()>;
}
