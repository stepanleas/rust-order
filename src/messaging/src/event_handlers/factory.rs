use crate::event_handlers::customer_event_handlers::CustomerCreatedEventHandler;
use crate::event_handlers::product_event_handlers::ProductCreatedEventHandler;
use crate::kafka::KafkaEventHandler;
use crate::kafka::topic::KafkaTopic;
use application::{CustomerMessageListener, ProductMessageListener};
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

    pub fn create(&self, topic: KafkaTopic) -> anyhow::Result<Arc<dyn KafkaEventHandler>> {
        let handler: Arc<dyn KafkaEventHandler> = match topic {
            KafkaTopic::CustomerCreated => Arc::new(CustomerCreatedEventHandler::new(Arc::clone(
                &self.customer_listener,
            ))),
            KafkaTopic::ProductCreated => Arc::new(ProductCreatedEventHandler::new(Arc::clone(
                &self.product_listener,
            ))),
        };

        Ok(handler)
    }
}
