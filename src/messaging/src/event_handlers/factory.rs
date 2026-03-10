use crate::event_handlers::customer_event_handlers::{
    CustomerCreatedEventHandler, CustomerUpdatedEventHandler,
};
use crate::event_handlers::product_event_handlers::{
    ProductCreatedEventHandler, ProductDeletedEventHandler, ProductUpdatedEventHandler,
};
use crate::kafka::KafkaEventHandler;
use crate::kafka::topic::KafkaTopic;
use application::ports::input::message::listeners::{
    CustomerMessageListener, ProductMessageListener,
};
use std::sync::Arc;

pub struct KafkaEventHandlerFactory {
    customer_listener: Arc<dyn CustomerMessageListener + Send + Sync>,
    product_listener: Arc<dyn ProductMessageListener + Send + Sync>,
}

impl KafkaEventHandlerFactory {
    pub fn new(
        customer_listener: Arc<dyn CustomerMessageListener + Send + Sync>,
        product_listener: Arc<dyn ProductMessageListener + Send + Sync>,
    ) -> Self {
        Self {
            customer_listener,
            product_listener,
        }
    }

    pub fn create(&self, topic: KafkaTopic) -> anyhow::Result<Box<dyn KafkaEventHandler>> {
        let handler: Box<dyn KafkaEventHandler> = match topic {
            KafkaTopic::CustomerCreated => Box::new(CustomerCreatedEventHandler::new(
                self.customer_listener.clone(),
            )),
            KafkaTopic::CustomerUpdated => Box::new(CustomerUpdatedEventHandler::new(
                self.customer_listener.clone(),
            )),
            KafkaTopic::ProductCreated => Box::new(ProductCreatedEventHandler::new(
                self.product_listener.clone(),
            )),
            KafkaTopic::ProductUpdated => Box::new(ProductUpdatedEventHandler::new(
                self.product_listener.clone(),
            )),
            KafkaTopic::ProductDeleted => Box::new(ProductDeletedEventHandler::new(
                self.product_listener.clone(),
            )),
        };

        Ok(handler)
    }
}
