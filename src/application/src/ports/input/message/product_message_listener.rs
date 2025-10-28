use crate::ProductRepository;
use crate::ports::input::message::listeners::ProductMessageListener;
use log::{error, info};
use std::sync::Arc;

pub struct ApplicationProductMessageListener {
    repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl ApplicationProductMessageListener {
    pub fn new(repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

impl ProductMessageListener for ApplicationProductMessageListener {
    fn product_created(&self, product: domain::Product) -> anyhow::Result<()> {
        let product_id = &product.id().as_uuid().to_string();

        match self.repository.save(product) {
            Ok(_) => {
                info!(
                    "Product is created in order database with id: {}",
                    product_id,
                );

                Ok(())
            }
            Err(error) => {
                error!(
                    "Error while saving product with id: {}. {}",
                    product_id,
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }

    fn product_updated(&self, product: domain::Product) -> anyhow::Result<()> {
        let product_id = &product.id().as_uuid().to_string();

        match self.repository.save(product) {
            Ok(_) => {
                info!(
                    "Product is updated in order database with id: {}",
                    product_id,
                );

                Ok(())
            }
            Err(error) => {
                error!(
                    "Error while updating product with id: {}. {}",
                    product_id,
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }
}
