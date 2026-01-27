#[cfg(test)]
mod tests {
    use crate::commands::{CreateOrderCommand, CreateOrderItemDto};
    use uuid::Uuid;

    #[test]
    fn test_create_order_command() -> anyhow::Result<()> {
        let customer_id = Uuid::new_v4();
        let price = 150.0;
        let items = vec![
            CreateOrderItemDto::new(Uuid::new_v4(), 2, 50.0, 100.0),
            CreateOrderItemDto::new(Uuid::new_v4(), 3, 25.0, 75.0),
        ];

        let command = CreateOrderCommand::new(customer_id, price, items);

        assert_eq!(command.customer_id(), customer_id);
        assert_eq!(command.price(), price);

        assert_eq!(2, command.items().len());
        assert_eq!(2, command.items()[0].quantity());
        assert_eq!(50.0, command.items()[0].price());
        assert_eq!(100.0, command.items()[0].sub_total());

        assert_eq!(3, command.items()[1].quantity());
        assert_eq!(25.0, command.items()[1].price());
        assert_eq!(75.0, command.items()[1].sub_total());

        Ok(())
    }
}
