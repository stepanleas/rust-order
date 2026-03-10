#[cfg(test)]
mod tests {
    use crate::config::configure;
    use crate::postgres_product_repository::PostgresProductRepository;
    use application::repositories::ProductRepository;
    use domain::entities::product::Product;
    use rusty_money::iso;
    use shared::domain::value_objects::ProductId;
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
            rusty_money::Money::from_str("30.0", iso::USD)?,
        ))?;

        let saved_product = ctx.repository.find_by_id(id)?;
        assert_eq!(id, saved_product.id());
        assert_eq!("Product 1", saved_product.title());
        assert_eq!(2, saved_product.quantity());
        assert_eq!("$30.00", saved_product.price().to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_update_product() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = ProductId::new();
        let product = Product::new(
            id,
            "Product 1".into(),
            2,
            rusty_money::Money::from_str("30.0", iso::USD)?,
        );
        ctx.repository.save(product)?;

        let updated_product = Product::builder()
            .id(id)
            .title("Updated Product".into())
            .quantity(5)
            .price(rusty_money::Money::from_str("50.0", iso::USD)?)
            .build();
        ctx.repository.save(updated_product)?;

        let saved_product = ctx.repository.find_by_id(id)?;
        assert_eq!(id, saved_product.id());
        assert_eq!("Updated Product", saved_product.title());
        assert_eq!(5, saved_product.quantity());
        assert_eq!("$50.00", saved_product.price().to_string());

        Ok(())
    }
}
