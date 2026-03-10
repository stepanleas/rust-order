use crate::kafka::KafkaEventHandler;
use crate::kafka::avro::customer_models::{
    CustomerCreatedEventAvroModel, CustomerUpdatedEventAvroModel,
};
use crate::mappers::CustomerMessagingMapper;
use anyhow::Result;
use application::ports::input::message::listeners::CustomerMessageListener;
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
        let reader = apache_avro::Reader::new(payload)?;

        for value in reader {
            let event: CustomerCreatedEventAvroModel = apache_avro::from_value(&value?)?;

            tracing::info!(
                "Received CustomerCreatedEvent: id={}, customer_id={}, user_name={}, created_at={}",
                event.id(),
                event.customer().id(),
                event.customer().user_name(),
                event.created_at(),
            );

            let customer = CustomerMessagingMapper::map_customer_avro_model_to_domain_entity(
                event.customer(),
            )?;

            match self.listener.customer_created(customer) {
                Ok(_) => {
                    tracing::info!(
                        "Successfully processed CustomerCreatedEvent for customer id: {}",
                        event.customer().id(),
                    );
                }
                Err(error) => {
                    tracing::error!(
                        "Error while processing CustomerCreatedEvent for customer id: {}. {}",
                        event.customer().id(),
                        error.to_string(),
                    );
                }
            }
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
        let reader = apache_avro::Reader::new(payload)?;

        for value in reader {
            let event: CustomerUpdatedEventAvroModel = apache_avro::from_value(&value?)?;

            tracing::info!(
                "Received CustomerUpdatedEvent: id={}, customer_id={}, user_name={}, created_at={}",
                event.id(),
                event.customer().id(),
                event.customer().user_name(),
                event.created_at(),
            );

            let customer = CustomerMessagingMapper::map_customer_avro_model_to_domain_entity(
                event.customer(),
            )?;

            match self.listener.customer_updated(customer) {
                Ok(_) => {
                    tracing::info!(
                        "Successfully processed CustomerUpdatedEvent for customer id: {}",
                        event.customer().id(),
                    );
                }
                Err(error) => {
                    tracing::error!(
                        "Error while processing CustomerUpdatedEvent for customer id: {}. {}",
                        event.customer().id(),
                        error.to_string(),
                    );
                }
            }
        }

        Ok(())
    }
}
