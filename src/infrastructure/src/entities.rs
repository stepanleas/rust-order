use crate::enums::OrderStatus;
use chrono::NaiveDateTime;
use diesel::internal::derives::multiconnection::bigdecimal::BigDecimal;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use domain::Order;
use domain::OrderItem;
use shared::domain::value_objects::Money;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct OrderEntity {
    id: Uuid,
    customer_id: Uuid,
    tracking_id: Uuid,
    price: BigDecimal,
    status: OrderStatus,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl OrderEntity {
    pub fn into(self, items: Vec<OrderItemEntity>) -> Order {
        Order::builder()
            .id(self.id)
            .customer_id(self.customer_id)
            .tracking_id(self.tracking_id)
            .price(Money::new(self.price))
            .items(items.into_iter().map(OrderItemEntity::into).collect())
            .status(self.status.into())
            .build()
    }
}

impl From<&Order> for OrderEntity {
    fn from(order: &Order) -> Self {
        Self {
            id: order.id(),
            customer_id: order.customer_id(),
            tracking_id: order.tracking_id(),
            price: order.price().clone().value(),
            status: order.status().clone().into(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Insertable, AsChangeset, PartialEq, Debug,
)]
#[diesel(belongs_to(OrderEntity, foreign_key=order_id))]
#[diesel(table_name = crate::schema::order_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct OrderItemEntity {
    id: Uuid,
    order_id: Uuid,
    product_id: Uuid,
    quantity: i32,
    price: BigDecimal,
    sub_total: BigDecimal,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Into<OrderItem> for OrderItemEntity {
    fn into(self) -> OrderItem {
        OrderItem::builder()
            .id(self.id)
            .order_id(self.order_id)
            .product_id(self.product_id)
            .quantity(self.quantity)
            .price(Money::new(self.price))
            .sub_total(Money::new(self.sub_total))
            .build()
    }
}

impl From<&OrderItem> for OrderItemEntity {
    fn from(item: &OrderItem) -> Self {
        Self {
            id: item.id(),
            order_id: item.order_id(),
            product_id: item.product_id(),
            quantity: item.quantity(),
            price: item.price().clone().value(),
            sub_total: item.sub_total().clone().value(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
