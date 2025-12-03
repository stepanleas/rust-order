#[cfg(test)]
mod tests {
    use crate::entities::customer::Customer;
    use shared::domain::value_objects::CustomerId;

    #[test]
    fn test_create_customer() -> anyhow::Result<()> {
        let customer_id = CustomerId::new();
        let customer = Customer::new(
            customer_id,
            "Artellas".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        );

        assert_eq!(customer.id(), customer_id);
        assert_eq!(customer.user_name(), "Artellas");
        assert_eq!(customer.first_name(), "John");
        assert_eq!(customer.last_name(), "Doe");

        Ok(())
    }
}
