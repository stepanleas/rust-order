use crate::entities::builders::OrderBuilder;
use crate::entities::order_item::OrderItem;
use crate::enums::OrderStatus;
use crate::error::DomainError;
use shared::domain::value_objects::{CustomerId, Money, OrderId};
use std::ops::Add;
use uuid::Uuid;

pub struct Order {
    id: OrderId,
    customer_id: CustomerId,
    tracking_id: Uuid,
    price: Money,
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl Order {
    pub fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }

    pub fn new(id: OrderId, customer_id: CustomerId) -> Self {
        let order = Order {
            id,
            customer_id,
            tracking_id: Uuid::new_v4(),
            price: Money::default(),
            items: vec![],
            status: OrderStatus::Pending,
        };

        order
    }

    pub fn id(&self) -> OrderId {
        self.id
    }

    pub fn customer_id(&self) -> CustomerId {
        self.customer_id
    }

    pub fn tracking_id(&self) -> Uuid {
        self.tracking_id
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn set_price(&mut self, price: Money) {
        self.price = price;
    }

    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<OrderItem>) {
        self.items = items;
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn mark_as_paid(&mut self) -> Result<(), DomainError> {
        if self.status != OrderStatus::Pending {
            return Err(DomainError::OrderDomainError {
                message: "Only pending orders can be marked as paid.".to_string(),
            });
        }

        self.status = OrderStatus::Paid;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        if self.status != OrderStatus::Pending {
            return Err(DomainError::OrderDomainError {
                message: "Order is not in correct state for initialization!".to_string(),
            });
        }

        if !self.price.is_greater_than_zero() {
            return Err(DomainError::OrderDomainError {
                message: "Total price must be greater than zero!".to_string(),
            });
        }

        self.validate_items_price()
    }

    fn validate_items_price(&self) -> Result<(), DomainError> {
        let order_items_total = self
            .items
            .iter()
            .map(|item| {
                if !item.is_price_valid() {
                    return Err(DomainError::OrderDomainError {
                        message: "Order item price is not valid!".to_string(),
                    });
                }

                Ok(item.sub_total().clone())
            })
            .collect::<Result<Vec<Money>, DomainError>>()?
            .into_iter()
            .fold(Money::zero(), |acc, m| acc.add(m));

        if self.price != order_items_total {
            return Err(DomainError::OrderDomainError {
                message: format!(
                    "Total price: {} is not equal to order items total: {}!",
                    self.price.clone().value(),
                    order_items_total.value(),
                ),
            });
        }

        Ok(())
    }
}
