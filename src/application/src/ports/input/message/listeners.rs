use domain::entities::customer::Customer;
use domain::entities::product::Product;
use mockall::automock;
use shared::domain::value_objects::ProductId;

#[automock]
pub trait CustomerMessageListener {
    fn customer_created(&self, customer: Customer) -> anyhow::Result<()>;
    fn customer_updated(&self, customer: Customer) -> anyhow::Result<()>;
}

#[automock]
pub trait ProductMessageListener {
    fn product_created(&self, product: Product) -> anyhow::Result<()>;
    fn product_updated(&self, product: Product) -> anyhow::Result<()>;
    fn product_deleted(&self, product_id: ProductId) -> anyhow::Result<()>;
}
