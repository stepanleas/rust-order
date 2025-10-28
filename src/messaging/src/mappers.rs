use crate::kafka::avro::customer_models::CustomerAvroModel;
use crate::kafka::avro::product_models::ProductAvroModel;
use domain::{Customer, Product};
use shared::domain::value_objects::{CustomerId, Money, ProductId};

pub struct CustomerMessagingMapper;

impl CustomerMessagingMapper {
    pub fn map_customer_avro_model_to_domain_entity(
        avro_model: &CustomerAvroModel,
    ) -> anyhow::Result<Customer> {
        let customer = Customer::builder()
            .id(CustomerId::from_str(avro_model.id())?)
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
        let price = Money::from_f64(avro_model.price().parse::<f64>()?)?;

        let product = domain::Product::builder()
            .id(ProductId::from_str(avro_model.id())?)
            .title(avro_model.title().into())
            .quantity(avro_model.quantity().into())
            .price(price)
            .build();

        Ok(product)
    }
}
