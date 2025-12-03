use crate::DbPool;
use crate::entities::ProductEntity;
use crate::schema::products::dsl::products;
use crate::schema::products::id;
use application::repositories::ProductRepository;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{OptionalExtension, RunQueryDsl};
use domain::entities::product::Product;
use domain::error::DomainError;
use shared::domain::value_objects::ProductId;

pub struct PostgresProductRepository {
    pool: DbPool,
}

impl PostgresProductRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl ProductRepository for PostgresProductRepository {
    fn find_by_id(&self, product_id: ProductId) -> anyhow::Result<Product> {
        let mut connection = self.pool.get()?;

        let product_entity = products
            .filter(id.eq(product_id.as_uuid()))
            .first::<ProductEntity>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound {
                message: format!("Could not find product with id: {}", product_id.as_uuid()),
            })?;

        Ok(product_entity.into())
    }

    fn find_by_ids(&self, product_ids: Vec<ProductId>) -> anyhow::Result<Vec<Product>> {
        let mut connection = self.pool.get()?;

        let product_entities: Vec<ProductEntity> = products
            .filter(id.eq_any(product_ids.iter().map(|product_id| product_id.as_uuid())))
            .load(&mut connection)?;

        Ok(product_entities
            .into_iter()
            .map(ProductEntity::into)
            .collect())
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

    fn delete(&self, product_id: ProductId) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        diesel::delete(products.filter(id.eq(product_id.as_uuid()))).execute(&mut connection)?;

        Ok(())
    }
}
