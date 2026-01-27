#[cfg(test)]
mod tests {
    use crate::entities::order::Order;
    use crate::entities::order_item::OrderItem;
    use crate::enums::OrderStatus;
    use shared::domain::value_objects::{CustomerId, Money, OrderId, OrderItemId, ProductId};
    use uuid::Uuid;

    fn make_item(order_id: OrderId, price: f64, quantity: i32) -> OrderItem {
        let sub_total = price * quantity as f64;

        OrderItem::new(
            OrderItemId::new(),
            order_id,
            ProductId::new(),
            quantity,
            Money::from_f64(price).unwrap(),
            Money::from_f64(sub_total).unwrap(),
        )
    }

    #[test]
    fn test_create_order() {
        let order_id = OrderId::new();
        let customer_id = CustomerId::new();

        let order = Order::new(order_id, customer_id);

        assert_eq!(order_id, order.id());
        assert_eq!(customer_id, order.customer_id());
        assert_ne!(Uuid::nil(), order.tracking_id());
        assert_eq!("0", order.price().to_string());
        assert_eq!(0, order.items().len());
        assert_eq!(OrderStatus::Pending, order.status());
    }

    #[test]
    fn test_validate_order_items_price_should_match_order_price() {
        let mut order = Order::new(OrderId::new(), CustomerId::new());
        order.set_price(Money::from_f64(120.0).unwrap());

        let first_item = make_item(order.id(), 10.0, 2);
        let second_item = make_item(order.id(), 25.0, 4);

        order.set_items(vec![first_item, second_item]);

        let result = order.validate();
        assert!(result.is_ok(), "Expected valid order to pass validation");
    }

    #[test]
    fn test_validate_order_items_price_should_not_match_order_price() {
        let mut order = Order::new(OrderId::new(), CustomerId::new());
        order.set_price(Money::from_f64(20.0).unwrap());

        let first_item = make_item(order.id(), 15.0, 2);
        let second_item = make_item(order.id(), 25.0, 4);

        order.set_items(vec![first_item, second_item]);

        let result = order.validate();
        assert!(result.is_err(), "Expected invalid order to pass validation");

        let message = result.unwrap_err().to_string();
        assert_eq!(
            "Total price: 20 is not equal to order items total: 130!",
            message,
        );
    }

    #[test]
    fn test_validate_invalid_sub_total_for_order_item() {
        let mut order = Order::new(OrderId::new(), CustomerId::new());
        order.set_price(Money::from_f64(20.0).unwrap());

        let invalid_item = OrderItem::new(
            OrderItemId::new(),
            order.id(),
            ProductId::new(),
            2,
            Money::from_f64(10.0).unwrap(),
            Money::from_f64(15.0).unwrap(),
        );

        order.set_items(vec![invalid_item]);

        let result = order.validate();
        assert!(result.is_err());

        let message = result.unwrap_err().to_string();
        assert_eq!("Order item price is not valid!", message);
    }
}
