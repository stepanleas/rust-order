use crate::ports::input::message::listeners::CustomerMessageListener;
use crate::repositories::CustomerRepository;
use domain::entities::customer::Customer;
use std::sync::Arc;

pub struct ApplicationCustomerMessageListener {
    repository: Arc<dyn CustomerRepository + Send + Sync>,
}

impl ApplicationCustomerMessageListener {
    pub fn new(repository: Arc<dyn CustomerRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

impl CustomerMessageListener for ApplicationCustomerMessageListener {
    fn customer_created(&self, customer: Customer) -> anyhow::Result<()> {
        let customer_id = &customer.id().as_uuid().to_string();

        match self.repository.save(customer) {
            Ok(_) => {
                tracing::info!(
                    "Customer is created in order database with id: {}",
                    customer_id,
                );

                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Error while saving customer with id: {}. {}",
                    customer_id,
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }

    fn customer_updated(&self, customer: Customer) -> anyhow::Result<()> {
        let customer_id = &customer.id().as_uuid().to_string();

        match self.repository.save(customer) {
            Ok(_) => {
                tracing::info!(
                    "Customer is updated in order database with id: {}",
                    customer_id,
                );

                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Error while updating customer with id: {}. {}",
                    customer_id,
                    error.to_string(),
                );

                Err(anyhow::anyhow!(error))
            }
        }
    }
}
