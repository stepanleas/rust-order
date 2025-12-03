#[cfg(test)]
mod tests {
    use crate::config::configure;
    use crate::postgres_product_repository::PostgresProductRepository;
    use application::repositories::ProductRepository;
    use domain::entities::product::Product;
    use shared::domain::value_objects::{Money, ProductId};
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Postgres>,
        repository: PostgresProductRepository,
    }

    async fn setup_context() -> anyhow::Result<TestContext> {
        let container = Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        let db_pool = configure(url).await?;

        Ok(TestContext {
            _container: container,
            repository: PostgresProductRepository::new(&db_pool),
        })
    }

    #[tokio::test]
    async fn test_create_product() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = ProductId::new();
        ctx.repository.save(Product::new(
            id,
            "Product 1".into(),
            2,
            Money::from_f64(30.0)?,
        ))?;

        let saved_product = ctx.repository.find_by_id(id)?;
        assert_eq!(saved_product.id(), id);
        assert_eq!(saved_product.title(), "Product 1");
        assert_eq!(saved_product.quantity(), 2);
        assert_eq!(saved_product.price(), &Money::from_f64(30.0)?);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_product() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = ProductId::new();
        let product = Product::new(id, "Product 1".into(), 2, Money::from_f64(30.0)?);
        ctx.repository.save(product)?;

        let updated_product = Product::builder()
            .id(id)
            .title("Updated Product".into())
            .quantity(5)
            .price(Money::from_f64(50.0)?)
            .build();
        ctx.repository.save(updated_product)?;

        let saved_product = ctx.repository.find_by_id(id)?;
        assert_eq!(saved_product.id(), id);
        assert_eq!(saved_product.title(), "Updated Product");
        assert_eq!(saved_product.quantity(), 5);
        assert_eq!(saved_product.price(), &Money::from_f64(50.0)?);

        Ok(())
    }
}
