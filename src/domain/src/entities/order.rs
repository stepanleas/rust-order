use crate::entities::builders::OrderBuilder;
use crate::entities::order_item::OrderItem;
use crate::enums::OrderStatus;
use crate::error::DomainError;
use rusty_money::iso::Currency;
use rusty_money::{Money, iso};
use shared::domain::value_objects::{CustomerId, OrderId};
use uuid::Uuid;

pub struct Order {
    id: OrderId,
    customer_id: CustomerId,
    tracking_id: Uuid,
    price: Money<'static, Currency>,
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl Order {
    pub fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }

    pub fn new(id: OrderId, customer_id: CustomerId) -> Self {
        Order {
            id,
            customer_id,
            tracking_id: Uuid::new_v4(),
            price: Money::from_minor(0, iso::USD),
            items: vec![],
            status: OrderStatus::Pending,
        }
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

    pub fn price(&self) -> &Money<'static, Currency> {
        &self.price
    }

    pub fn set_price(&mut self, price: Money<'static, Currency>) {
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

        if self.price.is_zero() {
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

                Ok(*item.sub_total())
            })
            .collect::<Result<Vec<Money<'static, Currency>>, DomainError>>()?
            .into_iter()
            .fold(Money::from_minor(0, iso::USD), |acc, m| {
                acc.add(m).unwrap_or(Money::from_minor(0, iso::USD))
            });

        if self.price != order_items_total {
            return Err(DomainError::OrderDomainError {
                message: format!(
                    "Total price: {} is not equal to order items total: {}!",
                    self.price, order_items_total,
                ),
            });
        }

        Ok(())
    }
}
