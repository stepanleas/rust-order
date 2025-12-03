use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KafkaTopic {
    CustomerCreated,
    CustomerUpdated,
    ProductCreated,
    ProductUpdated,
    ProductDeleted,
}

impl KafkaTopic {
    pub fn all() -> Vec<KafkaTopic> {
        vec![
            KafkaTopic::CustomerCreated,
            KafkaTopic::CustomerUpdated,
            KafkaTopic::ProductCreated,
            KafkaTopic::ProductUpdated,
            KafkaTopic::ProductDeleted,
        ]
    }
}

impl TryFrom<&str> for KafkaTopic {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "customer-created" => Ok(KafkaTopic::CustomerCreated),
            "customer-updated" => Ok(KafkaTopic::CustomerUpdated),
            "product-created" => Ok(KafkaTopic::ProductCreated),
            "product-updated" => Ok(KafkaTopic::ProductUpdated),
            "product-deleted" => Ok(KafkaTopic::ProductDeleted),
            _ => {
                tracing::error!("Unknown Kafka topic: {}", value);
                Err(anyhow::anyhow!("Unknown Kafka topic: {}", value))
            }
        }
    }
}

impl From<KafkaTopic> for String {
    fn from(topic: KafkaTopic) -> Self {
        match topic {
            KafkaTopic::CustomerCreated => "customer-created".to_string(),
            KafkaTopic::CustomerUpdated => "customer-updated".to_string(),
            KafkaTopic::ProductCreated => "product-created".to_string(),
            KafkaTopic::ProductUpdated => "product-updated".to_string(),
            KafkaTopic::ProductDeleted => "product-deleted".to_string(),
        }
    }
}

impl Display for KafkaTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let topic_str = match self {
            KafkaTopic::CustomerCreated => "customer-created",
            KafkaTopic::CustomerUpdated => "customer-updated",
            KafkaTopic::ProductCreated => "product-created",
            KafkaTopic::ProductUpdated => "product-updated",
            KafkaTopic::ProductDeleted => "product-deleted",
        };
        write!(f, "{}", topic_str)
    }
}
