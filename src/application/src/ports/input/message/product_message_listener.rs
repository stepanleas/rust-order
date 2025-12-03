use crate::ports::input::message::listeners::ProductMessageListener;
use crate::repositories::ProductRepository;
use domain::entities::product::Product;
use shared::domain::value_objects::ProductId;
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
    fn product_created(&self, product: Product) -> anyhow::Result<()> {
        let product_id = &product.id().as_uuid().to_string();

        match self.repository.save(product) {
            Ok(_) => {
                tracing::info!(
                    "Product is created in order database with id: {}",
                    product_id,
                );

                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Error while saving product with id: {}. {}",
                    product_id,
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }

    fn product_updated(&self, product: Product) -> anyhow::Result<()> {
        let product_id = &product.id().as_uuid().to_string();

        match self.repository.save(product) {
            Ok(_) => {
                tracing::info!(
                    "Product is updated in order database with id: {}",
                    product_id,
                );

                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Error while updating product with id: {}. {}",
                    product_id,
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }

    fn product_deleted(&self, product_id: ProductId) -> anyhow::Result<()> {
        match self.repository.delete(product_id) {
            Ok(_) => {
                tracing::info!(
                    "Product is updated in order database with id: {}",
                    product_id.as_uuid().to_string(),
                );

                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Error while updating product with id: {}. {}",
                    product_id.as_uuid().to_string(),
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }
}
