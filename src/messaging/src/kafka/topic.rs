use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum KafkaTopic {
    CustomerCreated,
    ProductCreated,
}

impl KafkaTopic {
    pub fn all() -> Vec<KafkaTopic> {
        vec![KafkaTopic::CustomerCreated, KafkaTopic::ProductCreated]
    }
}

impl TryFrom<&str> for KafkaTopic {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "customer-created" => Ok(KafkaTopic::CustomerCreated),
            "product-created" => Ok(KafkaTopic::ProductCreated),
            _ => Err(anyhow::anyhow!("Unknown Kafka topic: {}", value)),
        }
    }
}

impl Into<String> for KafkaTopic {
    fn into(self) -> String {
        match self {
            KafkaTopic::CustomerCreated => "customer-created".to_string(),
            KafkaTopic::ProductCreated => "product-created".to_string(),
        }
    }
}

impl Display for KafkaTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let topic_str = match self {
            KafkaTopic::CustomerCreated => "customer-created",
            KafkaTopic::ProductCreated => "product-created",
        };
        write!(f, "{}", topic_str)
    }
}
