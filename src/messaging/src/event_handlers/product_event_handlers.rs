use crate::kafka::KafkaEventHandler;
use crate::kafka::avro::product_models::{
    ProductCreatedEventAvroModel, ProductDeletedEventAvroModel, ProductUpdatedEventAvroModel,
};
use crate::mappers::ProductMessagingMapper;
use application::ports::input::message::listeners::ProductMessageListener;
use shared::domain::value_objects::ProductId;
use std::sync::Arc;

pub struct ProductCreatedEventHandler {
    listener: Arc<dyn ProductMessageListener + Sync + Send>,
}

impl ProductCreatedEventHandler {
    pub fn new(listener: Arc<dyn ProductMessageListener + Sync + Send>) -> Self {
        Self { listener }
    }
}

impl KafkaEventHandler for ProductCreatedEventHandler {
    fn handle_message(&self, payload: &[u8]) -> anyhow::Result<()> {
        let reader = apache_avro::Reader::new(payload)?;

        for value in reader {
            let event: ProductCreatedEventAvroModel = apache_avro::from_value(&value?)?;
            println!("{}", event.id());

            tracing::info!(
                "Received ProductCreatedEvent: id={}, product_id={} title={}, created_at={}",
                event.id(),
                event.product().id(),
                event.product().title(),
                event.created_at(),
            );

            let product =
                ProductMessagingMapper::map_product_avro_model_to_domain_entity(event.product())?;

            match self.listener.product_created(product) {
                Ok(_) => {
                    tracing::info!(
                        "Successfully processed ProductCreatedEvent for product id: {}",
                        event.product().id(),
                    );
                }
                Err(error) => {
                    tracing::error!(
                        "Error while processing ProductCreatedEvent for product id: {}. {}",
                        event.product().id(),
                        error.to_string(),
                    );
                }
            }
        }

        Ok(())
    }
}

pub struct ProductUpdatedEventHandler {
    listener: Arc<dyn ProductMessageListener + Sync + Send>,
}

impl ProductUpdatedEventHandler {
    pub fn new(listener: Arc<dyn ProductMessageListener + Sync + Send>) -> Self {
        Self { listener }
    }
}

impl KafkaEventHandler for ProductUpdatedEventHandler {
    fn handle_message(&self, payload: &[u8]) -> anyhow::Result<()> {
        let reader = apache_avro::Reader::new(payload)?;

        for value in reader {
            let event: ProductUpdatedEventAvroModel = apache_avro::from_value(&value?)?;

            tracing::info!(
                "Received ProductUpdatedEvent: id={}, product_id={}, title={}, created_at={}",
                event.id(),
                event.product().id(),
                event.product().title(),
                event.created_at(),
            );

            let product =
                ProductMessagingMapper::map_product_avro_model_to_domain_entity(event.product())?;

            match self.listener.product_updated(product) {
                Ok(_) => {
                    tracing::info!(
                        "Successfully processed ProductUpdatedEvent for product id: {}",
                        event.product().id(),
                    );
                }
                Err(error) => {
                    tracing::error!(
                        "Error while processing ProductUpdatedEvent for product id: {}. {}",
                        event.product().id(),
                        error.to_string(),
                    );
                }
            }
        }

        Ok(())
    }
}

pub struct ProductDeletedEventHandler {
    listener: Arc<dyn ProductMessageListener + Sync + Send>,
}

impl ProductDeletedEventHandler {
    pub fn new(listener: Arc<dyn ProductMessageListener + Sync + Send>) -> Self {
        Self { listener }
    }
}

impl KafkaEventHandler for ProductDeletedEventHandler {
    fn handle_message(&self, payload: &[u8]) -> anyhow::Result<()> {
        let reader = apache_avro::Reader::new(payload)?;

        for value in reader {
            let event: ProductDeletedEventAvroModel = apache_avro::from_value(&value?)?;

            tracing::info!(
                "Received ProductDeletedEvent: id={}, product_id={}, created_at={}",
                event.id(),
                event.product_id(),
                event.created_at(),
            );

            let product_id = match ProductId::from_str(event.product_id()) {
                Ok(id) => id,
                Err(error) => {
                    tracing::error!(
                        "Error while parsing ProductId from ProductDeletedEvent for product id: {}. {}",
                        event.product_id(),
                        error.to_string(),
                    );

                    return Err(anyhow::anyhow!(error));
                }
            };

            match self.listener.product_deleted(product_id) {
                Ok(_) => {
                    tracing::info!(
                        "Successfully processed ProductDeletedEvent for product id: {}",
                        event.product_id(),
                    );
                }
                Err(error) => {
                    tracing::error!(
                        "Error while processing ProductDeletedEvent for product id: {}. {}",
                        event.product_id(),
                        error.to_string(),
                    );
                }
            }
        }

        Ok(())
    }
}
