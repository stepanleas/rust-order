#[cfg(test)]
mod tests {
    use crate::commands::{CreateOrderCommand, CreateOrderItemDto};
    use crate::mappers::OrderMapper;
    use shared::domain::value_objects::Money;
    use uuid::Uuid;

    #[test]
    fn test_map_create_order_command_to_domain_entity() -> anyhow::Result<()> {
        let customer_id = Uuid::new_v4();
        let first_product_id = Uuid::new_v4();
        let second_product_id = Uuid::new_v4();

        let command = CreateOrderCommand::new(
            customer_id,
            450.0,
            vec![
                CreateOrderItemDto::new(first_product_id, 10, 30.0, 300.0),
                CreateOrderItemDto::new(second_product_id, 5, 30.0, 150.0),
            ],
        );

        let order = OrderMapper::map_create_order_command_to_domain_entity(command)?;

        assert_ne!(order.id().as_uuid().to_string(), Uuid::nil().to_string());
        assert_eq!(
            order.customer_id().as_uuid().to_string(),
            customer_id.to_string()
        );
        assert_eq!(order.price(), &Money::from_f64(450.0)?);
        assert_eq!(order.items().len(), 2);

        assert_ne!(
            order.items()[0].id().as_uuid().to_string(),
            Uuid::nil().to_string()
        );
        assert_eq!(order.items()[0].order_id(), order.id());
        assert_eq!(
            order.items()[0].product_id().as_uuid().to_string(),
            first_product_id.to_string()
        );
        assert_eq!(order.items()[0].quantity(), 10);
        assert_eq!(order.items()[0].price(), &Money::from_f64(30.0)?);
        assert_eq!(order.items()[0].sub_total(), &Money::from_f64(300.0)?);

        assert_ne!(
            order.items()[1].id().as_uuid().to_string(),
            Uuid::nil().to_string()
        );
        assert_eq!(order.items()[1].order_id(), order.id());
        assert_eq!(
            order.items()[1].product_id().as_uuid().to_string(),
            second_product_id.to_string()
        );
        assert_eq!(order.items()[1].quantity(), 5);
        assert_eq!(order.items()[1].price(), &Money::from_f64(30.0)?);
        assert_eq!(order.items()[1].sub_total(), &Money::from_f64(150.0)?);

        Ok(())
    }
}
