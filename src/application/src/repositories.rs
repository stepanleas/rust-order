use domain::{Customer, Order};
use uuid::Uuid;

pub trait OrderRepository {
    fn find_by_tracking_id(&self, tracking_id: Uuid) -> anyhow::Result<Order>;
    fn save(&self, entity: &Order) -> anyhow::Result<()>;
}

pub trait CustomerRepository {
    fn find_by_id(&self, customer_id: Uuid) -> anyhow::Result<Customer>;
    fn save(&self, entity: Customer) -> anyhow::Result<()>;
}
