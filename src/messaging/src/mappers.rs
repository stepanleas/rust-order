use crate::kafka::avro::customer_models::CustomerAvroModel;
use crate::kafka::avro::product_models::ProductAvroModel;
use domain::entities::customer::Customer;
use domain::entities::product::Product;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rusty_money::{Money, iso};
use shared::domain::value_objects::{CustomerId, ProductId};

pub struct CustomerMessagingMapper;

impl CustomerMessagingMapper {
    pub fn map_customer_avro_model_to_domain_entity(
        avro_model: &CustomerAvroModel,
    ) -> anyhow::Result<Customer> {
        let customer_id = match CustomerId::from_str(avro_model.id()) {
            Ok(id) => id,
            Err(error) => {
                anyhow::bail!(
                    "Failed to parse CustomerId from Avro model id '{}': {}",
                    avro_model.id(),
                    error,
                );
            }
        };

        let customer = Customer::builder()
            .id(customer_id)
            .user_name(avro_model.user_name().into())
            .first_name(avro_model.first_name().into())
            .last_name(avro_model.last_name().into())
            .build();

        Ok(customer)
    }
}

pub struct ProductMessagingMapper;

impl ProductMessagingMapper {
    pub fn map_product_avro_model_to_domain_entity(
        avro_model: &ProductAvroModel,
    ) -> anyhow::Result<Product> {
        let price = match Decimal::from_f64(avro_model.price().parse::<f64>()?) {
            Some(decimal) => decimal,
            None => {
                anyhow::bail!(
                    "Failed to convert price '{}' to Decimal for Avro model id '{}'",
                    avro_model.price(),
                    avro_model.id(),
                );
            }
        };

        let product_id = match ProductId::from_str(avro_model.id()) {
            Ok(id) => id,
            Err(error) => {
                anyhow::bail!(
                    "Failed to parse ProductId from Avro model id '{}': {}",
                    avro_model.id(),
                    error,
                );
            }
        };

        let product = Product::builder()
            .id(product_id)
            .title(avro_model.title().into())
            .quantity(avro_model.quantity())
            .price(Money::from_decimal(price, iso::USD))
            .build();

        Ok(product)
    }
}
