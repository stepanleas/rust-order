use crate::DbPool;
use crate::entities::CustomerEntity;
use crate::schema::customers::dsl::customers;
use crate::schema::customers::id;
use application::CustomerRepository;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{OptionalExtension, RunQueryDsl};
use domain::{Customer, DomainError};
use uuid::Uuid;

pub struct PostgresCustomerRepository {
    pool: DbPool,
}

impl PostgresCustomerRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl CustomerRepository for PostgresCustomerRepository {
    fn find_by_id(&self, entity_id: Uuid) -> anyhow::Result<Customer> {
        let mut connection = self.pool.get()?;

        let ticket_entity = customers
            .filter(id.eq(entity_id))
            .first::<CustomerEntity>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound { id: entity_id })?;

        Ok(ticket_entity.into())
    }

    fn save(&self, entity: Customer) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        let persistent_entity = CustomerEntity::from(entity);

        diesel::insert_into(customers)
            .values(&persistent_entity)
            .on_conflict(id)
            .do_update()
            .set(&persistent_entity)
            .execute(&mut connection)?;

        Ok(())
    }
}
