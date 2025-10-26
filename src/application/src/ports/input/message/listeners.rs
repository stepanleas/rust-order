use crate::CustomerRepository;
use domain::Customer;
use log::{error, info};
use std::sync::Arc;

pub trait CustomerMessageListener {
    fn customer_created(&self, customer: Customer) -> anyhow::Result<()>;
}

pub struct CustomerMessageListenerImpl {
    repository: Arc<dyn CustomerRepository + Send + Sync>,
}

impl CustomerMessageListenerImpl {
    pub fn new(repository: Arc<dyn CustomerRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

impl CustomerMessageListener for CustomerMessageListenerImpl {
    fn customer_created(&self, customer: Customer) -> anyhow::Result<()> {
        let customer_id = &customer.id().as_uuid().to_string();

        match self.repository.save(customer) {
            Ok(_) => {
                info!(
                    "Customer is created in order database with id: {}",
                    customer_id,
                );

                Ok(())
            }
            Err(error) => {
                error!(
                    "Error while saving customer with id: {}. {}",
                    customer_id,
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }
}

pub trait ProductMessageListener {
    fn product_created(&self, product: domain::Product) -> anyhow::Result<()>;
}

pub struct ProductMessageListenerImpl {
    repository: Arc<dyn crate::ProductRepository + Send + Sync>,
}

impl ProductMessageListenerImpl {
    pub fn new(repository: Arc<dyn crate::ProductRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

impl ProductMessageListener for ProductMessageListenerImpl {
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
}
