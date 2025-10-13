use crate::DbPool;
use crate::entities::{OrderEntity, OrderItemEntity};
use crate::schema::order_items::dsl::order_items;
use crate::schema::orders::dsl::orders;
use application::OrderRepository;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{BelongingToDsl, ExpressionMethods, OptionalExtension};
use domain::{DomainError, Order};
use uuid::Uuid;

pub struct PostgresOrderRepository {
    pool: DbPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl OrderRepository for PostgresOrderRepository {
    fn find_by_tracking_id(&self, tracking_id: Uuid) -> anyhow::Result<Order> {
        let mut connection = self.pool.get()?;

        let order_entity = orders
            .filter(crate::schema::orders::tracking_id.eq(tracking_id))
            .first::<OrderEntity>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound { id: tracking_id })?;

        let order_item_entities = OrderItemEntity::belonging_to(&order_entity)
            .load::<OrderItemEntity>(&mut connection)?;

        Ok(order_entity.into(order_item_entities))
    }

    fn save(&self, entity: &Order) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        connection.transaction::<(), Error, _>(|conn| {
            let order_entity = OrderEntity::from(entity);
            diesel::insert_into(orders)
                .values(&order_entity)
                .execute(conn)?;

            let order_item_entities: Vec<OrderItemEntity> =
                entity.items().iter().map(OrderItemEntity::from).collect();

            diesel::insert_into(order_items)
                .values(&order_item_entities)
                .execute(conn)?;

            Ok(())
        })?;

        Ok(())
    }
}
