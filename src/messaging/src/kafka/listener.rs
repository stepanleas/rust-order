use crate::event_handlers::KafkaEventHandlerFactory;
use crate::kafka::consumer::KafkaConsumer;
use crate::kafka::topic::KafkaTopic;
use kafka::consumer::{Message, MessageSet};
use std::sync::{Mutex, MutexGuard};
use std::thread;
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

    pub fn listen(&self) {
        let mut consumer = self.consumer.lock().unwrap_or_else(|p| p.into_inner());
        tracing::info!("Kafka listener started...");

        let mut backoff = Duration::from_secs(1);

        loop {
            match consumer.poll() {
                Ok(message_sets) => {
                    for message_set in message_sets.iter() {
                        self.process_message_set(message_set, &mut consumer);
                    }
                    consumer.commit_consumed().unwrap_or_else(|error| {
                        tracing::error!("Failed to commit consumed messages: {}", error)
                    });
                }
                Err(e) => {
                    tracing::warn!("Kafka poll failed: {}. Retrying in {:?}...", e, backoff);
                    thread::sleep(backoff);
                    backoff = std::cmp::min(backoff * 2, Duration::from_secs(30));
                }
            }
        }
    }

    fn process_message_set(
        &self,
        message_set: MessageSet,
        consumer: &mut MutexGuard<KafkaConsumer>,
    ) {
        let message_set_topic = message_set.topic();

        let topic = match KafkaTopic::try_from(message_set_topic) {
            Ok(topic) => topic,
            Err(_) => {
                tracing::error!(
                    "Received message set for unknown topic: {}. Committing offset anyway.",
                    message_set_topic,
                );

                if let Err(err) = consumer.consume_messageset(message_set) {
                    tracing::error!(
                        "Failed to consume (commit) message set for unknown topic {}: {}",
                        message_set_topic,
                        err,
                    );
                }

                return;
            }
        };

        for message in message_set.messages() {
            self.process_message(topic, message);
        }

        if let Err(err) = consumer.consume_messageset(message_set) {
            tracing::error!("Failed to consume message set for topic {}: {}", topic, err);
        }
    }

    fn process_message(&self, topic: KafkaTopic, message: &Message) {
        match self.factory.create(topic) {
            Ok(handler) => match handler.handle_message(message.value) {
                Ok(_) => tracing::info!("Message processed for topic {}", topic),
                Err(error) => {
                    tracing::error!("Failed to handle message for topic {}: {}", topic, error)
                }
            },
            Err(e) => {
                tracing::warn!("No handler for topic '{}': {}", topic, e);
            }
        }
    }
}
