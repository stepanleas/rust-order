use log::error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum KafkaTopic {
    CustomerCreated,
    CustomerUpdated,
    ProductCreated,
    ProductUpdated,
}

impl KafkaTopic {
    pub fn all() -> Vec<KafkaTopic> {
        vec![
            KafkaTopic::CustomerCreated,
            KafkaTopic::CustomerUpdated,
            KafkaTopic::ProductCreated,
            KafkaTopic::ProductUpdated,
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
            _ => {
                error!("Unknown Kafka topic: {}", value);
                Err(anyhow::anyhow!("Unknown Kafka topic: {}", value))
            }
        }
    }
}

impl Into<String> for KafkaTopic {
    fn into(self) -> String {
        match self {
            KafkaTopic::CustomerCreated => "customer-created".to_string(),
            KafkaTopic::CustomerUpdated => "customer-updated".to_string(),
            KafkaTopic::ProductCreated => "product-created".to_string(),
            KafkaTopic::ProductUpdated => "product-updated".to_string(),
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
        };
        write!(f, "{}", topic_str)
    }
}
