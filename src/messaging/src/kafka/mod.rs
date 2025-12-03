pub mod avro;
pub mod consumer;
pub mod listener;
pub mod topic;

pub trait KafkaEventHandler: Send + Sync {
    fn handle_message(&self, payload: &[u8]) -> anyhow::Result<()>;
}
