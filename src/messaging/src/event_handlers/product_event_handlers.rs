use crate::kafka::KafkaEventHandler;
use crate::kafka::avro::product_models::{
    ProductCreatedEventAvroModel, ProductUpdatedEventAvroModel,
};
use crate::mappers::ProductMessagingMapper;
use apache_avro::{Reader, from_value};
use application::ProductMessageListener;
use log::info;
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
        let reader = Reader::new(payload)?;

        for value in reader {
            let event: ProductCreatedEventAvroModel = from_value(&value?)?;
            info!(
                "Received ProductCreatedEvent: id={} title={}",
                event.product().id(),
                event.product().title()
            );

            let product =
                ProductMessagingMapper::map_product_avro_model_to_domain_entity(event.product())?;
            self.listener.product_created(product)?;
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
        let reader = Reader::new(payload)?;

        for value in reader {
            let event: ProductUpdatedEventAvroModel = from_value(&value?)?;
            info!(
                "Received ProductUpdatedEvent: id={} title={}",
                event.product().id(),
                event.product().title()
            );

            let product =
                ProductMessagingMapper::map_product_avro_model_to_domain_entity(event.product())?;
            self.listener.product_updated(product)?;
        }

        Ok(())
    }
}
