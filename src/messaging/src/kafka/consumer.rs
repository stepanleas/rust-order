use kafka::client::{FetchOffset, GroupOffsetStorage, KafkaClient};
use kafka::consumer::{Consumer, MessageSet, MessageSets};

pub struct KafkaConsumer {
    consumer: Consumer,
}

impl KafkaConsumer {
    pub fn new(client: KafkaClient, group_id: String, topic: String) -> anyhow::Result<Self> {
        let consumer = Consumer::from_client(client)
            .with_group(group_id)
            .with_topic(topic)
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka.into())
            .with_fetch_min_bytes(1)
            .create()?;

        Ok(Self { consumer })
    }

    pub fn poll(&mut self) -> kafka::Result<MessageSets> {
        self.consumer.poll()
    }

    pub fn consume_messageset(&mut self, message_set: MessageSet) -> kafka::error::Result<()> {
        self.consumer.consume_messageset(message_set)
    }

    pub fn commit_consumed(&mut self) -> kafka::error::Result<()> {
        self.consumer.commit_consumed()
    }
}
