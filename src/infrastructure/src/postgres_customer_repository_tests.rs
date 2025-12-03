#[cfg(test)]
mod tests {
    use crate::config::configure;
    use crate::postgres_customer_repository::PostgresCustomerRepository;
    use application::repositories::CustomerRepository;
    use domain::entities::customer::Customer;
    use shared::domain::value_objects::CustomerId;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Postgres>,
        repository: PostgresCustomerRepository,
    }

    async fn setup_context() -> anyhow::Result<TestContext> {
        let container = Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        let db_pool = configure(url).await?;
        let repository = PostgresCustomerRepository::new(&db_pool);

        Ok(TestContext {
            _container: container,
            repository,
        })
    }

    #[tokio::test]
    async fn test_create_customer() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = CustomerId::new();
        let customer = Customer::new(id, "Artellas".into(), "Mike".into(), "Dane".into());
        ctx.repository.save(customer)?;

        let saved_customer = ctx.repository.find_by_id(id)?;
        assert_eq!(saved_customer.id(), id);
        assert_eq!(saved_customer.user_name(), "Artellas");
        assert_eq!(saved_customer.first_name(), "Mike");
        assert_eq!(saved_customer.last_name(), "Dane");

        Ok(())
    }

    #[tokio::test]
    async fn test_update_customer() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = CustomerId::new();
        let customer = Customer::new(id, "Artellas".into(), "Mike".into(), "Dane".into());
        ctx.repository.save(customer)?;

        let updated_customer = Customer::builder()
            .id(id)
            .user_name("Updated username".into())
            .first_name("Updated first name".into())
            .last_name("Updated last name".into())
            .build();
        ctx.repository.save(updated_customer)?;

        let saved_customer = ctx.repository.find_by_id(id)?;
        assert_eq!(saved_customer.id(), id);
        assert_eq!(saved_customer.user_name(), "Updated username");
        assert_eq!(saved_customer.first_name(), "Updated first name");
        assert_eq!(saved_customer.last_name(), "Updated last name");

        Ok(())
    }
}
