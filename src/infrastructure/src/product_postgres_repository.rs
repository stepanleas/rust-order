use crate::DbPool;
use crate::entities::ProductEntity;
use crate::schema::products::dsl::products;
use crate::schema::products::id;
use application::ProductRepository;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{OptionalExtension, RunQueryDsl};
use domain::{DomainError, Product};
use uuid::Uuid;

pub struct PostgresProductRepository {
    pool: DbPool,
}

impl PostgresProductRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl ProductRepository for PostgresProductRepository {
    fn find_by_id(&self, entity_id: Uuid) -> anyhow::Result<Product> {
        let mut connection = self.pool.get()?;

        let ticket_entity = products
            .filter(id.eq(entity_id))
            .first::<ProductEntity>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound { id: entity_id })?;

        Ok(ticket_entity.into())
    }

    fn save(&self, entity: Product) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        let persistent_entity = ProductEntity::from(entity);

        diesel::insert_into(products)
            .values(&persistent_entity)
            .on_conflict(id)
            .do_update()
            .set(&persistent_entity)
            .execute(&mut connection)?;

        Ok(())
    }
}
