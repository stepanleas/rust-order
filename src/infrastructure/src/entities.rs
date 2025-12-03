use crate::enums::OrderStatus;
use chrono::NaiveDateTime;
use diesel::internal::derives::multiconnection::bigdecimal::BigDecimal;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use domain::entities::customer::Customer;
use domain::entities::order::Order;
use domain::entities::order_item::OrderItem;
use domain::entities::product::Product;
use shared::domain::value_objects::{CustomerId, Money, OrderId, OrderItemId, ProductId};
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
            .id(OrderId::from_uuid(self.id))
            .customer_id(CustomerId::from_uuid(self.customer_id))
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
            id: order.id().into(),
            customer_id: order.customer_id().into(),
            tracking_id: order.tracking_id(),
            price: order.price().clone().value(),
            status: order.status().into(),
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

impl From<OrderItemEntity> for OrderItem {
    fn from(entity: OrderItemEntity) -> Self {
        OrderItem::builder()
            .id(OrderItemId::from_uuid(entity.id))
            .order_id(OrderId::from_uuid(entity.order_id))
            .product_id(ProductId::from_uuid(entity.product_id))
            .quantity(entity.quantity)
            .price(Money::new(entity.price))
            .sub_total(Money::new(entity.sub_total))
            .build()
    }
}

impl From<&OrderItem> for OrderItemEntity {
    fn from(item: &OrderItem) -> Self {
        Self {
            id: item.id().into(),
            order_id: item.order_id().into(),
            product_id: item.product_id().into(),
            quantity: item.quantity(),
            price: item.price().clone().value(),
            sub_total: item.sub_total().clone().value(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct CustomerEntity {
    id: Uuid,
    user_name: String,
    first_name: String,
    last_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Customer> for CustomerEntity {
    fn from(customer: Customer) -> Self {
        Self {
            id: customer.id().into(),
            user_name: customer.user_name().into(),
            first_name: customer.first_name().into(),
            last_name: customer.last_name().into(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<CustomerEntity> for Customer {
    fn from(entity: CustomerEntity) -> Self {
        Customer::builder()
            .id(CustomerId::from_uuid(entity.id))
            .user_name(entity.user_name)
            .first_name(entity.first_name)
            .last_name(entity.last_name)
            .build()
    }
}

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct ProductEntity {
    id: Uuid,
    title: String,
    quantity: i32,
    price: BigDecimal,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Product> for ProductEntity {
    fn from(product: Product) -> Self {
        Self {
            id: product.id().into(),
            title: product.title().into(),
            quantity: product.quantity(),
            price: product.price().clone().value(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<ProductEntity> for Product {
    fn from(entity: ProductEntity) -> Self {
        Product::builder()
            .id(ProductId::from_uuid(entity.id))
            .title(entity.title)
            .quantity(entity.quantity)
            .price(Money::new(entity.price))
            .build()
    }
}
