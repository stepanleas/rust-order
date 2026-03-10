use crate::commands::{CreateOrderCommand, CreateOrderItemDto};
use crate::mappers::OrderMapper;
use crate::repositories::{CustomerRepository, OrderRepository, ProductRepository};
use domain::entities::product::Product;
use domain::error::DomainError;
use domain::events::OrderCreatedEvent;
use shared::domain::value_objects::{CustomerId, ProductId};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateOrderCommandHandler {
    pub order_repository: Arc<dyn OrderRepository>,
    pub customer_repository: Arc<dyn CustomerRepository>,
    pub product_repository: Arc<dyn ProductRepository>,
}

impl CreateOrderCommandHandler {
    pub fn new(
        order_repository: Arc<dyn OrderRepository>,
        customer_repository: Arc<dyn CustomerRepository>,
        product_repository: Arc<dyn ProductRepository>,
    ) -> Self {
        Self {
            order_repository,
            customer_repository,
            product_repository,
        }
    }

    pub async fn execute(&self, command: CreateOrderCommand) -> anyhow::Result<Uuid> {
        self.check_customer(command.customer_id())?;
        self.check_products_quantity(command.items())?;

        let order = OrderMapper::map_create_order_command_to_domain_entity(command)?;
        order.validate()?;

        self.order_repository.save(&order)?;
        tracing::info!("Order created with id: {:?}", order.id());
        let event = OrderCreatedEvent::new(order);

        Ok(event.order().tracking_id())
    }

    fn check_customer(&self, customer_id: Uuid) -> anyhow::Result<()> {
        match self
            .customer_repository
            .find_by_id(CustomerId::from_uuid(customer_id))
        {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow::anyhow!(DomainError::OrderDomainError {
                message: error.to_string(),
            })),
        }
    }

    fn retrieve_products(&self, items: &[CreateOrderItemDto]) -> anyhow::Result<Vec<Product>> {
        let product_ids: Vec<Uuid> = items.iter().map(|item| item.product_id()).collect();
        let products: Vec<Product> = self
            .product_repository
            .find_by_ids(
                product_ids
                    .iter()
                    .map(|product_id| ProductId::from_uuid(*product_id))
                    .collect(),
            )?
            .into_iter()
            .collect();

        let requested: HashSet<_> = product_ids.iter().cloned().collect();
        let found: HashSet<_> = products
            .iter()
            .map(|product| *product.id().as_uuid())
            .collect();

        let missing: Vec<_> = requested.difference(&found).collect();

        if !missing.is_empty() {
            return Err(anyhow::anyhow!(DomainError::OrderDomainError {
                message: format!("Missing product ids {:?}", missing).to_string(),
            }));
        }

        Ok(products)
    }

    fn check_products_quantity(&self, items: &[CreateOrderItemDto]) -> anyhow::Result<()> {
        let products = self.retrieve_products(items)?;

        let products_map: HashMap<Uuid, Product> = products
            .into_iter()
            .map(|product| (*product.id().as_uuid(), product))
            .collect();

        for item in items {
            if let Some(product) = products_map.get(&item.product_id())
                && item.quantity() > product.quantity()
            {
                return Err(anyhow::anyhow!(DomainError::OrderDomainError {
                    message: format!(
                        "Insufficient quantity for product {}: requested {}, available {}",
                        product.title(),
                        item.quantity(),
                        product.quantity(),
                    ),
                }));
            }
        }

        Ok(())
    }
}
