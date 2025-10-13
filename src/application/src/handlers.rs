use crate::OrderRepository;
use crate::commands::CreateOrderCommand;
use domain::{Order, OrderCreatedEvent, OrderItem};
use shared::domain::value_objects::Money;
use std::sync::Arc;
use anyhow::Context;
use uuid::Uuid;

pub struct CreateOrderCommandHandler {
    pub repository: Arc<dyn OrderRepository>,
}

impl CreateOrderCommandHandler {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateOrderCommand) -> anyhow::Result<Uuid> {
        let price = Money::from_f64(command.price())
            .context("Invalid order price!")?;

        let mut order = Order::builder()
            .id(Uuid::new_v4())
            .customer_id(command.customer_id())
            .price(price)
            .build();

        let order_items = command
            .items()
            .iter()
            .map(|item| -> anyhow::Result<OrderItem> {
                let price = Money::from_f64(item.price())
                    .context("Invalid order price for!")?;
                let sub_total = Money::from_f64(item.sub_total())
                    .context("Invalid order item sub total price!")?;

                Ok(OrderItem::builder()
                    .id(Uuid::new_v4())
                    .order_id(order.id())
                    .product_id(item.product_id())
                    .quantity(item.quantity())
                    .price(price)
                    .sub_total(sub_total)
                    .build())
            })
            .collect::<Result<Vec<_>, _>>()?;

        order.set_items(order_items);
        order.validate()?;

        let event = OrderCreatedEvent::new(order);
        self.repository.save(&event.order)?;

        Ok(event.order.tracking_id())
    }
}
