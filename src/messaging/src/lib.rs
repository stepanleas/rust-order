use ::kafka::client::KafkaClient;
use application::CustomerMessageListener;
use std::sync::Arc;

mod kafka;

pub fn listen(
    client: KafkaClient,
    customer_message_listener: Arc<dyn CustomerMessageListener>,
) -> anyhow::Result<()> {
    let consumer = kafka::KafkaConsumer::new(
        client,
        "order-service-group".to_string(),
        "customer.created".to_string(),
    )?;

    let customer_listener = kafka::CustomerKafkaListener::new(consumer, customer_message_listener);

    customer_listener.listen()?;

    Ok(())
}
