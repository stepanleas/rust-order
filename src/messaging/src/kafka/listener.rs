use crate::event_handlers::KafkaEventHandlerFactory;
use crate::kafka::consumer::KafkaConsumer;
use crate::kafka::topic::KafkaTopic;
use kafka::consumer::Message;
use log::{error, info, warn};
use std::sync::Mutex;
use std::time::Duration;

pub struct KafkaMessageListener {
    consumer: Mutex<KafkaConsumer>,
    factory: KafkaEventHandlerFactory,
}

impl KafkaMessageListener {
    pub fn new(consumer: KafkaConsumer, factory: KafkaEventHandlerFactory) -> Self {
        Self {
            consumer: Mutex::new(consumer),
            factory,
        }
    }

    pub fn listen(&self) -> anyhow::Result<()> {
        let mut consumer = self.consumer.lock().unwrap_or_else(|p| p.into_inner());
        info!("Kafka listener started...");

        loop {
            match consumer.poll() {
                Ok(message_sets) => {
                    for message_set in message_sets.iter() {
                        let topic = KafkaTopic::try_from(message_set.topic())?;

                        for message in message_set.messages() {
                            self.process_message(topic, message)?;
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

    fn process_message(&self, topic: KafkaTopic, message: &Message) -> anyhow::Result<()> {
        match self.factory.create(topic) {
            Ok(handler) => {
                match handler.handle_message(message.value) {
                    Ok(_) => info!("Message processed for topic {}", topic),
                    Err(e) => error!("Failed to handle message for {}: {}", topic, e),
                }

                Ok(())
            }
            Err(e) => {
                warn!("No handler for topic '{}': {}", topic, e);
                Ok(())
            }
        }
    }
}
