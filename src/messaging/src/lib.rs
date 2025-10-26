use crate::event_handlers::KafkaEventHandlerFactory;
use crate::kafka::KafkaMessageListener;
use ::kafka::client::KafkaClient;

pub mod event_handlers;
mod kafka;
mod mappers;

pub fn listen(
    client: KafkaClient,
    factory: KafkaEventHandlerFactory,
    group_id: String,
) -> anyhow::Result<()> {
    let consumer = kafka::KafkaConsumer::new(client, group_id)?;
    let kafka_listener = KafkaMessageListener::new(consumer, factory);

    kafka_listener.listen()?;

    Ok(())
}
