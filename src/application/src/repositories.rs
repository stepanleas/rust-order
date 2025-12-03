use domain::entities::customer::Customer;
use domain::entities::order::Order;
use domain::entities::product::Product;
use mockall::automock;
use shared::domain::value_objects::{CustomerId, ProductId};
use uuid::Uuid;

#[automock]
pub trait OrderRepository {
    fn find_by_tracking_id(&self, tracking_id: Uuid) -> anyhow::Result<Order>;
    fn save(&self, entity: &Order) -> anyhow::Result<()>;
}

#[automock]
pub trait CustomerRepository {
    fn find_by_id(&self, customer_id: CustomerId) -> anyhow::Result<Customer>;
    fn save(&self, entity: Customer) -> anyhow::Result<()>;
}

#[automock]
pub trait ProductRepository {
    fn find_by_ids(&self, product_ids: Vec<ProductId>) -> anyhow::Result<Vec<Product>>;
    fn find_by_id(&self, product_id: ProductId) -> anyhow::Result<Product>;
    fn save(&self, product: Product) -> anyhow::Result<()>;
    fn delete(&self, product_id: ProductId) -> anyhow::Result<()>;
}
