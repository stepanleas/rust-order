use application::{CustomerRepository, OrderRepository, Settings};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub order_repository: Arc<dyn OrderRepository + Send + Sync>,
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
}
