#[cfg(test)]
mod tests {
    use crate::Order;
    use crate::entities::order_item::OrderItem;
    use shared::domain::value_objects::Money;
    use std::str::FromStr;
    use uuid::Uuid;

    fn make_item(order_id: Uuid, price: f64, quantity: i32) -> OrderItem {
        let sub_total = price * quantity as f64;

        OrderItem::new(
            Uuid::new_v4(),
            order_id,
            Uuid::new_v4(),
            quantity,
            Money::from_f64(price).unwrap(),
            Money::from_f64(sub_total).unwrap(),
        )
    }

    #[test]
    fn test_validate_order_items_price_should_match_order_price() {
        let order_id =
            Uuid::from_str("d01f002b-b0af-49cd-9707-3e451f53ade5").expect("Error parsing uuid");
        let customer_id =
            Uuid::from_str("19b0755d-6ca3-4e4f-809c-9b92695d2929").expect("Error parsing uuid");

        let mut order = Order::new(order_id, customer_id);
        order.set_price(Money::from_f64(120.0).unwrap());

        let first_item = make_item(order.id(), 10.0, 2);
        let second_item = make_item(order.id(), 25.0, 4);

        order.set_items(vec![first_item, second_item]);

        let result = order.validate();
        assert!(result.is_ok(), "Expected valid order to pass validation");
    }

    #[test]
    fn test_validate_order_items_price_should_not_match_order_price() {
        let order_id =
            Uuid::from_str("d01f002b-b0af-49cd-9707-3e451f53ade5").expect("Error parsing uuid");
        let customer_id =
            Uuid::from_str("19b0755d-6ca3-4e4f-809c-9b92695d2929").expect("Error parsing uuid");

        let mut order = Order::new(order_id, customer_id);
        order.set_price(Money::from_f64(20.0).unwrap());

        let first_item = make_item(order.id(), 15.0, 2);
        let second_item = make_item(order.id(), 25.0, 4);

        order.set_items(vec![first_item, second_item]);

        let result = order.validate();
        assert!(result.is_err(), "Expected invalid order to pass validation");
    }

    #[test]
    fn test_validate_invalid_sub_total_for_order_item() {
        let order_id =
            Uuid::from_str("d01f002b-b0af-49cd-9707-3e451f53ade5").expect("Error parsing uuid");
        let customer_id =
            Uuid::from_str("19b0755d-6ca3-4e4f-809c-9b92695d2929").expect("Error parsing uuid");

        let mut order = Order::new(order_id, customer_id);
        order.set_price(Money::from_f64(20.0).unwrap());

        let invalid_item = OrderItem::new(
            Uuid::new_v4(),
            order.id(),
            Uuid::new_v4(),
            2,
            Money::from_f64(10.0).unwrap(),
            Money::from_f64(15.0).unwrap(),
        );

        order.set_items(vec![invalid_item]);

        let result = order.validate();
        assert!(result.is_err());
    }
}
