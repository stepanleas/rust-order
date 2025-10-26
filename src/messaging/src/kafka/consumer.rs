use crate::kafka::topic::KafkaTopic;
use kafka::client::{FetchOffset, GroupOffsetStorage, KafkaClient};
use kafka::consumer::{Consumer, MessageSet, MessageSets};

pub struct KafkaConsumer {
    consumer: Consumer,
}

impl KafkaConsumer {
    pub fn new(client: KafkaClient, group_id: String) -> anyhow::Result<Self> {
        let mut consumer_builder = Consumer::from_client(client)
            .with_group(group_id)
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka.into())
            .with_fetch_min_bytes(1);

        for topic in KafkaTopic::all() {
            consumer_builder = consumer_builder.with_topic(topic.into());
        }

        let consumer = consumer_builder.create()?;
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
