pub mod avro;
mod consumer;
mod listener;
pub mod topic;

pub use consumer::KafkaConsumer;
pub use listener::KafkaMessageListener;

pub trait KafkaEventHandler: Send + Sync {
    fn handle_message(&self, payload: &[u8]) -> anyhow::Result<()>;
}
