#[cfg(test)]
mod tests {
    use crate::commands::{CreateOrderCommand, CreateOrderItemDto};
    use crate::handlers::CreateOrderCommandHandler;
    use crate::repositories::{MockCustomerRepository, MockOrderRepository, MockProductRepository};
    use domain::entities::customer::Customer;
    use domain::entities::order::Order;
    use domain::entities::product::Product;
    use mockall::predicate;
    use rust_decimal::dec;
    use rusty_money::{Money, iso};
    use shared::domain::value_objects::{CustomerId, ProductId};
    use std::sync::Arc;
    use uuid::Uuid;

    fn create_customer(customer_id: CustomerId) -> Customer {
        Customer::new(customer_id, "Artellas".into(), "Mike".into(), "Dane".into())
    }

    fn create_product(product_id: ProductId, quantity: i32) -> anyhow::Result<Product> {
        Ok(Product::new(
            product_id,
            "Sample Product".into(),
            quantity,
            Money::from_str("30.0", iso::USD)?,
        ))
    }

    #[tokio::test]
    async fn test_create_order_command_handler_execute() -> anyhow::Result<()> {
        let customer_id = CustomerId::new();

        let first_product_id = ProductId::new();
        let second_product_id = ProductId::new();

        let mut mock_order_repository = MockOrderRepository::new();
        let mut mock_customer_repository = MockCustomerRepository::new();
        let mut mock_product_repository = MockProductRepository::new();

        mock_customer_repository
            .expect_find_by_id()
            .once()
            .with(predicate::eq(customer_id))
            .returning(move |_| Ok(create_customer(customer_id)));

        mock_product_repository
            .expect_find_by_ids()
            .once()
            .with(predicate::eq(vec![first_product_id, second_product_id]))
            .returning(move |_| {
                Ok(vec![
                    create_product(first_product_id, 10)?,
                    create_product(second_product_id, 5)?,
                ])
            });

        mock_order_repository
            .expect_save()
            .once()
            .withf(move |order: &Order| {
                order.customer_id() == customer_id
                    && order.items().len() == 2
                    && order.price().to_string() == "$450.00"
            })
            .returning(|_| Ok(()));

        let items = vec![
            CreateOrderItemDto::new(*first_product_id.as_uuid(), 10, dec!(30.0), dec!(300.0)),
            CreateOrderItemDto::new(*second_product_id.as_uuid(), 5, dec!(30.0), dec!(150.0)),
        ];
        let command = CreateOrderCommand::new(*customer_id.as_uuid(), dec!(450.0), items);

        let handler = CreateOrderCommandHandler::new(
            Arc::new(mock_order_repository),
            Arc::new(mock_customer_repository),
            Arc::new(mock_product_repository),
        );

        let tracking_id = handler.execute(command).await?;

        assert_ne!(Uuid::nil(), tracking_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_order_command_handler_execute_with_non_existent_customer()
    -> anyhow::Result<()> {
        let customer_id = CustomerId::new();

        let first_product_id = ProductId::new();
        let second_product_id = ProductId::new();

        let mut mock_order_repository = MockOrderRepository::new();
        let mut mock_customer_repository = MockCustomerRepository::new();
        let mut mock_product_repository = MockProductRepository::new();

        mock_customer_repository
            .expect_find_by_id()
            .once()
            .with(predicate::eq(customer_id))
            .returning(move |_| {
                Err(anyhow::anyhow!(
                    "Customer with id {:?} not found",
                    customer_id.as_uuid(),
                ))
            });

        mock_product_repository.expect_find_by_ids().never();

        mock_order_repository.expect_save().never();

        let items = vec![
            CreateOrderItemDto::new(*first_product_id.as_uuid(), 10, dec!(30.0), dec!(300.0)),
            CreateOrderItemDto::new(*second_product_id.as_uuid(), 5, dec!(30.0), dec!(150.0)),
        ];
        let command = CreateOrderCommand::new(*customer_id.as_uuid(), dec!(450.0), items);

        let handler = CreateOrderCommandHandler::new(
            Arc::new(mock_order_repository),
            Arc::new(mock_customer_repository),
            Arc::new(mock_product_repository),
        );

        let result = handler.execute(command).await;
        assert!(result.is_err());

        let message = result.unwrap_err().to_string();
        assert_eq!(
            format!("Customer with id {:?} not found", customer_id.as_uuid()).as_str(),
            message,
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_create_order_command_handler_execute_with_different_quantity()
    -> anyhow::Result<()> {
        let customer_id = CustomerId::new();

        let first_product_id = ProductId::new();
        let second_product_id = ProductId::new();

        let mut mock_order_repository = MockOrderRepository::new();
        let mut mock_customer_repository = MockCustomerRepository::new();
        let mut mock_product_repository = MockProductRepository::new();

        mock_customer_repository
            .expect_find_by_id()
            .once()
            .with(predicate::eq(customer_id))
            .returning(move |_| Ok(create_customer(customer_id)));

        mock_product_repository
            .expect_find_by_ids()
            .once()
            .with(predicate::eq(vec![first_product_id, second_product_id]))
            .returning(move |_| {
                Ok(vec![
                    create_product(first_product_id, 10)?,
                    create_product(second_product_id, 5)?,
                ])
            });

        mock_order_repository.expect_save().never();

        let items = vec![
            CreateOrderItemDto::new(*first_product_id.as_uuid(), 12, dec!(30.0), dec!(300.0)),
            CreateOrderItemDto::new(*second_product_id.as_uuid(), 5, dec!(30.0), dec!(150.0)),
        ];
        let command = CreateOrderCommand::new(*customer_id.as_uuid(), dec!(450.0), items);

        let handler = CreateOrderCommandHandler::new(
            Arc::new(mock_order_repository),
            Arc::new(mock_customer_repository),
            Arc::new(mock_product_repository),
        );

        let result = handler.execute(command).await;
        assert!(result.is_err());

        let message = result.unwrap_err().to_string();
        assert_eq!(
            "Insufficient quantity for product Sample Product: requested 12, available 10",
            message,
        );

        Ok(())
    }
}
