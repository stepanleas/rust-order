#[cfg(test)]
mod tests {
    use crate::config::configure;
    use crate::postgres_customer_repository::PostgresCustomerRepository;
    use crate::postgres_order_repository::PostgresOrderRepository;
    use crate::postgres_product_repository::PostgresProductRepository;
    use application::repositories::{CustomerRepository, OrderRepository, ProductRepository};
    use domain::entities::customer::Customer;
    use domain::entities::order::Order;
    use domain::entities::order_item::OrderItem;
    use domain::entities::product::Product;
    use shared::domain::value_objects::{CustomerId, Money, OrderId, OrderItemId, ProductId};
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;
    use uuid::Uuid;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Postgres>,
        product_repository: PostgresProductRepository,
        customer_repository: PostgresCustomerRepository,
        order_repository: PostgresOrderRepository,
    }

    async fn setup_context() -> anyhow::Result<TestContext> {
        let container = Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        let db_pool = configure(url).await?;

        Ok(TestContext {
            _container: container,
            product_repository: PostgresProductRepository::new(&db_pool),
            customer_repository: PostgresCustomerRepository::new(&db_pool),
            order_repository: PostgresOrderRepository::new(&db_pool),
        })
    }

    #[tokio::test]
    async fn test_create_order() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let order_id = OrderId::new();
        let customer_id = CustomerId::new();

        let first_product_id = ProductId::new();
        let second_product_id = ProductId::new();
        let first_order_item_id = OrderItemId::new();
        let second_order_item_id = OrderItemId::new();

        ctx.product_repository.save(Product::new(
            first_product_id,
            "Product 1".into(),
            2,
            Money::from_f64(30.0)?,
        ))?;
        ctx.product_repository.save(Product::new(
            second_product_id,
            "Product 2".into(),
            5,
            Money::from_f64(30.0)?,
        ))?;

        ctx.customer_repository.save(Customer::new(
            customer_id,
            "Artellas".into(),
            "Mike".into(),
            "Dane".into(),
        ))?;

        let order = Order::builder()
            .id(order_id)
            .customer_id(customer_id)
            .items(vec![
                OrderItem::new(
                    first_order_item_id,
                    order_id,
                    ProductId::new(),
                    2,
                    Money::from_f64(30.0)?,
                    Money::from_f64(60.0)?,
                ),
                OrderItem::new(
                    second_order_item_id,
                    order_id,
                    ProductId::new(),
                    3,
                    Money::from_f64(30.0)?,
                    Money::from_f64(90.0)?,
                ),
            ])
            .build();

        ctx.order_repository.save(&order)?;

        let saved_order = ctx
            .order_repository
            .find_by_tracking_id(order.tracking_id())?;
        assert_eq!(saved_order.id(), order_id);
        assert_eq!(saved_order.customer_id(), customer_id);
        assert_eq!(saved_order.items().len(), 2);

        assert_eq!(saved_order.items()[0].id(), first_order_item_id);
        assert_eq!(saved_order.items()[0].order_id(), order_id);
        assert_ne!(
            saved_order.items()[0].product_id().as_uuid().to_string(),
            Uuid::nil().to_string()
        );
        assert_eq!(saved_order.items()[0].quantity(), 2);
        assert_eq!(saved_order.items()[0].price(), &Money::from_f64(30.0)?);
        assert_eq!(saved_order.items()[0].sub_total(), &Money::from_f64(60.0)?);

        assert_eq!(saved_order.items()[1].id(), second_order_item_id);
        assert_eq!(saved_order.items()[1].order_id(), order_id);
        assert_ne!(
            saved_order.items()[1].product_id().as_uuid().to_string(),
            Uuid::nil().to_string()
        );
        assert_eq!(saved_order.items()[1].quantity(), 3);
        assert_eq!(saved_order.items()[1].price(), &Money::from_f64(30.0)?);
        assert_eq!(saved_order.items()[1].sub_total(), &Money::from_f64(90.0)?);

        Ok(())
    }
}
