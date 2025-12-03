#[cfg(test)]
mod tests {
    use crate::event_handlers::customer_event_handlers::CustomerCreatedEventHandler;
    use crate::kafka::KafkaEventHandler;
    use apache_avro::Schema;
    use apache_avro::types::Value;
    use application::ports::input::message::listeners::MockCustomerMessageListener;
    use std::sync::Arc;

    #[test]
    fn test_customer_created_event_handler_handle_message() -> anyhow::Result<()> {
        let mut mock_listener = MockCustomerMessageListener::new();

        mock_listener
            .expect_customer_created()
            .once()
            .withf(|customer| {
                customer.user_name() == "Artellas"
                    && customer.first_name() == "Mike"
                    && customer.last_name() == "Dane"
            })
            .returning(|_| Ok(()));

        let event_value = Value::Record(vec![
            ("id".into(), Value::String("evt-123".into())),
            (
                "customer".into(),
                Value::Record(vec![
                    (
                        "id".into(),
                        Value::String("0411a4a9-1edb-4180-9556-800cb2b84721".into()),
                    ),
                    ("user_name".into(), Value::String("Artellas".into())),
                    ("first_name".into(), Value::String("Mike".into())),
                    ("last_name".into(), Value::String("Dane".into())),
                ]),
            ),
            (
                "created_at".into(),
                Value::String("2025-01-01T12:00:00Z".into()),
            ),
        ]);

        let avro_payload = parse_schema(
            include_str!("../../avro/schemas/customer_created_event_avro_model.avsc"),
            event_value,
        )?;

        let handler = CustomerCreatedEventHandler::new(Arc::new(mock_listener));
        handler.handle_message(&avro_payload)?;

        Ok(())
    }

    #[test]
    fn test_customer_created_event_handler_handle_message_with_invalid_id() -> anyhow::Result<()> {
        let mut mock_listener = MockCustomerMessageListener::new();

        mock_listener.expect_customer_created().never();

        let event_value = Value::Record(vec![
            ("id".into(), Value::String("evt-123".into())),
            (
                "customer".into(),
                Value::Record(vec![
                    ("id".into(), Value::String("customer-123".into())),
                    ("user_name".into(), Value::String("Artellas".into())),
                    ("first_name".into(), Value::String("Mike".into())),
                    ("last_name".into(), Value::String("Dane".into())),
                ]),
            ),
            (
                "created_at".into(),
                Value::String("2025-01-01T12:00:00Z".into()),
            ),
        ]);

        let avro_payload = parse_schema(
            include_str!("../../avro/schemas/customer_created_event_avro_model.avsc"),
            event_value,
        )?;

        let handler = CustomerCreatedEventHandler::new(Arc::new(mock_listener));
        let result = handler.handle_message(&avro_payload);

        assert!(result.is_err());

        let message = result.unwrap_err().to_string();
        assert_eq!(
            "Failed to parse CustomerId from Avro model id 'customer-123': invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `u` at 2",
            message,
        );

        Ok(())
    }

    fn parse_schema(schema_path: &str, value: Value) -> anyhow::Result<Vec<u8>> {
        let schema = Schema::parse_str(schema_path)?;
        let mut writer = apache_avro::Writer::new(&schema, Vec::new());

        writer.append(value)?;
        writer.flush()?;

        Ok(writer.into_inner()?)
    }
}
