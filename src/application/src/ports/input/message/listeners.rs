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
