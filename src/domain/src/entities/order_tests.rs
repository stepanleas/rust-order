#[cfg(test)]
mod tests {
    use crate::entities::order::Order;
    use crate::entities::order_item::OrderItem;
    use crate::enums::OrderStatus;
    use rusty_money::{Money, iso};
    use shared::domain::value_objects::{CustomerId, OrderId, OrderItemId, ProductId};
    use uuid::Uuid;

    fn make_item(order_id: OrderId, price: &str, quantity: i32) -> anyhow::Result<OrderItem> {
        let price = Money::from_str(price, iso::USD)?;
        let sub_total = price.mul(quantity)?;

        Ok(OrderItem::new(
            OrderItemId::new(),
            order_id,
            ProductId::new(),
            quantity,
            price,
            sub_total,
        ))
    }

    #[test]
    fn test_create_order() -> anyhow::Result<()> {
        let order_id = OrderId::new();
        let customer_id = CustomerId::new();

        let order = Order::new(order_id, customer_id);

        assert_eq!(order_id, order.id());
        assert_eq!(customer_id, order.customer_id());
        assert_ne!(Uuid::nil(), order.tracking_id());
        assert_eq!("$0.00", order.price().to_string());
        assert_eq!(0, order.items().len());
        assert_eq!(OrderStatus::Pending, order.status());

        Ok(())
    }

    #[test]
    fn test_validate_order_items_price_should_match_order_price() -> anyhow::Result<()> {
        let mut order = Order::new(OrderId::new(), CustomerId::new());
        order.set_price(Money::from_str("120.0", iso::USD)?);

        let first_item = make_item(order.id(), "10.0", 2)?;
        let second_item = make_item(order.id(), "25.0", 4)?;

        order.set_items(vec![first_item, second_item]);

        let result = order.validate();
        assert!(result.is_ok(), "Expected valid order to pass validation");

        Ok(())
    }

    #[test]
    fn test_validate_order_items_price_should_not_match_order_price() -> anyhow::Result<()> {
        let mut order = Order::new(OrderId::new(), CustomerId::new());
        order.set_price(Money::from_str("20.0", iso::USD)?);

        let first_item = make_item(order.id(), "15.0", 2)?;
        let second_item = make_item(order.id(), "25.0", 4)?;

        order.set_items(vec![first_item, second_item]);

        let result = order.validate();
        assert!(result.is_err(), "Expected invalid order to pass validation");

        let message = result.unwrap_err().to_string();
        assert_eq!(
            "Total price: $20.00 is not equal to order items total: $130.00!",
            message,
        );

        Ok(())
    }

    #[test]
    fn test_validate_invalid_sub_total_for_order_item() -> anyhow::Result<()> {
        let mut order = Order::new(OrderId::new(), CustomerId::new());
        order.set_price(Money::from_str("20.0", iso::USD)?);

        let invalid_item = OrderItem::new(
            OrderItemId::new(),
            order.id(),
            ProductId::new(),
            2,
            Money::from_str("10.0", iso::USD).unwrap_or(Money::from_minor(0, iso::USD)),
            Money::from_str("15.0", iso::USD).unwrap_or(Money::from_minor(0, iso::USD)),
        );

        order.set_items(vec![invalid_item]);

        let result = order.validate();
        assert!(result.is_err());

        let message = result.unwrap_err().to_string();
        assert_eq!("Order item price is not valid!", message);

        Ok(())
    }
}
