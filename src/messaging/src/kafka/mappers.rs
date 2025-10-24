use crate::kafka::avro::CustomerAvroModel;
use domain::Customer;
use shared::domain::value_objects::CustomerId;

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
