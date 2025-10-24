use crate::commands::CreateOrderCommand;
use crate::mappers::OrderMapper;
use crate::{CustomerRepository, OrderRepository};
use domain::OrderCreatedEvent;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateOrderCommandHandler {
    pub order_repository: Arc<dyn OrderRepository>,
    pub customer_repository: Arc<dyn CustomerRepository>,
}

impl CreateOrderCommandHandler {
    pub fn new(
        order_repository: Arc<dyn OrderRepository>,
        customer_repository: Arc<dyn CustomerRepository>,
    ) -> Self {
        Self {
            order_repository,
            customer_repository,
        }
    }

    pub async fn execute(&self, command: CreateOrderCommand) -> anyhow::Result<Uuid> {
        self.customer_repository.find_by_id(command.customer_id())?;
        let order = OrderMapper::map_create_order_command_to_domain_entity(command)?;

        order.validate()?;

        self.order_repository.save(&order)?;
        println!("Order created with id: {:?}", order.id());
        let event = OrderCreatedEvent::new(order);

        Ok(event.order().tracking_id())
    }
}
