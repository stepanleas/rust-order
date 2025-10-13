use domain::Order;
use uuid::Uuid;

pub trait OrderRepository {
    fn find_by_tracking_id(&self, tracking_id: Uuid) -> anyhow::Result<Order>;
    fn save(&self, entity: &Order) -> anyhow::Result<()>;
}
