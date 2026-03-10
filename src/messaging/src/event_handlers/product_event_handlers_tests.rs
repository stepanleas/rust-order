#[cfg(test)]
mod tests {
    use crate::event_handlers::product_event_handlers::{
        ProductCreatedEventHandler, ProductUpdatedEventHandler,
    };
    use crate::kafka::KafkaEventHandler;
    use apache_avro::Schema;
    use apache_avro::types::Value;
    use application::ports::input::message::listeners::MockProductMessageListener;
    use std::sync::Arc;

    #[test]
    fn test_product_created_event_handler_handle_message() -> anyhow::Result<()> {
        let mut mock_listener = MockProductMessageListener::new();

        mock_listener
            .expect_product_created()
            .once()
            .withf(|product| {
                product.id().as_uuid().to_string() == "0411a4a9-1edb-4180-9556-800cb2b84721"
                    && product.title() == "Laptop"
                    && product.quantity() == 10
                    && product.price().to_string() == "$99.99"
            })
            .returning(|_| Ok(()));

        let event_value = Value::Record(vec![
            (
                "id".into(),
                Value::String("eb0163b3-cf88-4053-8c96-986b4f5b9f2e".into()),
            ),
            (
                "product".into(),
                Value::Record(vec![
                    (
                        "id".into(),
                        Value::String("0411a4a9-1edb-4180-9556-800cb2b84721".into()),
                    ),
                    ("title".into(), Value::String("Laptop".into())),
                    ("quantity".into(), Value::Int(10)),
                    ("price".into(), Value::String("99.99".into())),
                ]),
            ),
            (
                "created_at".into(),
                Value::String("2025-01-01T12:00:00Z".into()),
            ),
        ]);

        let avro_payload = parse_schema(
            include_str!("../../avro/schemas/product_created_event_avro_model.avsc"),
            event_value,
        )?;

        let handler = ProductCreatedEventHandler::new(Arc::new(mock_listener));
        handler.handle_message(&avro_payload)?;

        Ok(())
    }

    #[test]
    fn test_product_created_event_handler_handle_message_with_invalid_id() -> anyhow::Result<()> {
        let mut mock_listener = MockProductMessageListener::new();

        mock_listener.expect_product_created().never();

        let event_value = Value::Record(vec![
            (
                "id".into(),
                Value::String("eb0163b3-cf88-4053-8c96-986b4f5b9f2e".into()),
            ),
            (
                "product".into(),
                Value::Record(vec![
                    ("id".into(), Value::String("product-123".into())),
                    ("title".into(), Value::String("Laptop".into())),
                    ("quantity".into(), Value::Int(10)),
                    ("price".into(), Value::String("99.99".into())),
                ]),
            ),
            (
                "created_at".into(),
                Value::String("2025-01-01T12:00:00Z".into()),
            ),
        ]);

        let avro_payload = parse_schema(
            include_str!("../../avro/schemas/product_created_event_avro_model.avsc"),
            event_value,
        )?;

        let handler = ProductCreatedEventHandler::new(Arc::new(mock_listener));
        let result = handler.handle_message(&avro_payload);

        assert!(result.is_err());

        let message = result.unwrap_err().to_string();
        assert_eq!(
            "Failed to parse ProductId from Avro model id 'product-123': invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `p` at 1",
            message,
        );

        Ok(())
    }

    #[test]
    fn test_product_updated_event_handler_handle_message() -> anyhow::Result<()> {
        let mut mock_listener = MockProductMessageListener::new();

        mock_listener
            .expect_product_updated()
            .once()
            .withf(|product| {
                product.id().as_uuid().to_string() == "0411a4a9-1edb-4180-9556-800cb2b84721"
                    && product.title() == "Laptop"
                    && product.quantity() == 10
                    && product.price().to_string() == "$99.99"
            })
            .returning(|_| Ok(()));

        let event_value = Value::Record(vec![
            (
                "id".into(),
                Value::String("eb0163b3-cf88-4053-8c96-986b4f5b9f2e".into()),
            ),
            (
                "product".into(),
                Value::Record(vec![
                    (
                        "id".into(),
                        Value::String("0411a4a9-1edb-4180-9556-800cb2b84721".into()),
                    ),
                    ("title".into(), Value::String("Laptop".into())),
                    ("quantity".into(), Value::Int(10)),
                    ("price".into(), Value::String("99.99".into())),
                ]),
            ),
            (
                "created_at".into(),
                Value::String("2025-01-01T12:00:00Z".into()),
            ),
        ]);

        let avro_payload = parse_schema(
            include_str!("../../avro/schemas/product_updated_event_avro_model.avsc"),
            event_value,
        )?;

        let handler = ProductUpdatedEventHandler::new(Arc::new(mock_listener));
        handler.handle_message(&avro_payload)?;

        Ok(())
    }

    #[test]
    fn test_product_updated_event_handler_handle_message_with_invalid_id() -> anyhow::Result<()> {
        let mut mock_listener = MockProductMessageListener::new();

        mock_listener.expect_product_updated().never();

        let event_value = Value::Record(vec![
            (
                "id".into(),
                Value::String("eb0163b3-cf88-4053-8c96-986b4f5b9f2e".into()),
            ),
            (
                "product".into(),
                Value::Record(vec![
                    ("id".into(), Value::String("product-123".into())),
                    ("title".into(), Value::String("Laptop".into())),
                    ("quantity".into(), Value::Int(10)),
                    ("price".into(), Value::String("99.99".into())),
                ]),
            ),
            (
                "created_at".into(),
                Value::String("2025-01-01T12:00:00Z".into()),
            ),
        ]);

        let avro_payload = parse_schema(
            include_str!("../../avro/schemas/product_updated_event_avro_model.avsc"),
            event_value,
        )?;

        let handler = ProductUpdatedEventHandler::new(Arc::new(mock_listener));
        let result = handler.handle_message(&avro_payload);

        assert!(result.is_err());

        let message = result.unwrap_err().to_string();
        assert_eq!(
            "Failed to parse ProductId from Avro model id 'product-123': invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `p` at 1",
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
