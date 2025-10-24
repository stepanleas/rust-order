use crate::{CreateOrderCommand, CreateOrderItemDto};
use anyhow::Context;
use domain::{Order, OrderItem};
use shared::domain::value_objects::{CustomerId, Money, OrderId, OrderItemId, ProductId};

pub struct OrderMapper;

impl OrderMapper {
    pub fn map_create_order_command_to_domain_entity(
        command: CreateOrderCommand,
    ) -> anyhow::Result<Order> {
        let price = Money::from_f64(command.price()).context("Invalid order price!")?;

        let mut order = Order::builder()
            .id(OrderId::new())
            .customer_id(CustomerId::from_uuid(command.customer_id()))
            .price(price)
            .build();

        let order_items =
            Self::map_create_order_item_dto_to_domain_entity(command.items(), order.id())?;
        order.set_items(order_items);

        Ok(order)
    }

    fn map_create_order_item_dto_to_domain_entity(
        items: &[CreateOrderItemDto],
        order_id: OrderId,
    ) -> anyhow::Result<Vec<OrderItem>> {
        let order_items = items
            .iter()
            .map(|item| {
                let price = Money::from_f64(item.price()).context("Invalid order price for!")?;
                let sub_total = Money::from_f64(item.sub_total())
                    .context("Invalid order item sub total price!")?;

                let order_item = OrderItem::builder()
                    .id(OrderItemId::new())
                    .order_id(order_id)
                    .product_id(ProductId::from_uuid(item.product_id()))
                    .quantity(item.quantity())
                    .price(price)
                    .sub_total(sub_total)
                    .build();

                Ok(order_item)
            })
            .collect::<anyhow::Result<Vec<OrderItem>>>()?;

        Ok(order_items)
    }
}
