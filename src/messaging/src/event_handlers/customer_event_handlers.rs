use crate::kafka::KafkaEventHandler;
use crate::kafka::avro::customer_models::CustomerCreatedEventAvroModel;
use crate::mappers::CustomerMessagingMapper;
use anyhow::Result;
use apache_avro::{Reader, from_value};
use application::CustomerMessageListener;
use log::info;
use std::sync::Arc;

pub struct CustomerCreatedEventHandler {
    listener: Arc<dyn CustomerMessageListener + Send + Sync>,
}

impl CustomerCreatedEventHandler {
    pub fn new(listener: Arc<dyn CustomerMessageListener + Send + Sync>) -> Self {
        Self { listener }
    }
}

impl KafkaEventHandler for CustomerCreatedEventHandler {
    fn handle_message(&self, payload: &[u8]) -> Result<()> {
        let reader = Reader::new(payload)?;

        for value in reader {
            let event: CustomerCreatedEventAvroModel = from_value(&value?)?;

            info!(
                "Received CustomerCreatedEvent: id={} user_name={} created_at={}",
                event.customer().id(),
                event.customer().user_name(),
                event.created_at(),
            );

            let customer = CustomerMessagingMapper::map_customer_avro_model_to_domain_entity(
                event.customer(),
            )?;
            self.listener.customer_created(customer)?;
        }

        Ok(())
    }
}

pub struct CustomerUpdatedEventHandler {
    listener: Arc<dyn CustomerMessageListener + Send + Sync>,
}

impl CustomerUpdatedEventHandler {
    pub fn new(listener: Arc<dyn CustomerMessageListener + Send + Sync>) -> Self {
        Self { listener }
    }
}

impl KafkaEventHandler for CustomerUpdatedEventHandler {
    fn handle_message(&self, payload: &[u8]) -> Result<()> {
        let reader = Reader::new(payload)?;

        for value in reader {
            let event: CustomerCreatedEventAvroModel = from_value(&value?)?;

            info!(
                "Received CustomerUpdatedEvent: id={} user_name={} updated_at={}",
                event.customer().id(),
                event.customer().user_name(),
                event.created_at(),
            );

            let customer = CustomerMessagingMapper::map_customer_avro_model_to_domain_entity(
                event.customer(),
            )?;
            self.listener.customer_updated(customer)?;
        }

        Ok(())
    }
}
