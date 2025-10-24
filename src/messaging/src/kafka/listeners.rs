use crate::kafka::avro::CustomerCreatedEventAvroModel;
use crate::kafka::consumer::KafkaConsumer;
use crate::kafka::mappers::CustomerMessagingMapper;
use apache_avro::{Reader, from_value};
use application::CustomerMessageListener;
use log::{error, info};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct CustomerKafkaListener {
    consumer: Mutex<KafkaConsumer>,
    listener: Arc<dyn CustomerMessageListener>,
}

impl CustomerKafkaListener {
    pub fn new(consumer: KafkaConsumer, listener: Arc<dyn CustomerMessageListener>) -> Self {
        Self {
            consumer: Mutex::new(consumer),
            listener,
        }
    }

    pub fn listen(&self) -> anyhow::Result<()> {
        let mut consumer = self
            .consumer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        info!("Kafka listener started...");

        loop {
            match consumer.poll() {
                Ok(message_sets) => {
                    for message_set in message_sets.iter() {
                        for message in message_set.messages() {
                            if let Err(e) = self.handle_message(message.value) {
                                error!("Failed to handle message: {}", e);
                            }
                        }
                        consumer.consume_messageset(message_set)?;
                    }
                    consumer.commit_consumed()?;
                }
                Err(e) => {
                    error!("Kafka poll error: {}", e);
                    std::thread::sleep(Duration::from_secs(2));
                }
            }
        }
    }

    fn handle_message(&self, payload: &[u8]) -> anyhow::Result<()> {
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
