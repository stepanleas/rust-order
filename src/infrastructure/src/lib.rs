mod config;
mod customer_postgres_repository;
mod entities;
mod enums;
mod order_postgres_repository;
mod outbox;
mod product_postgres_repository;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use config::configure;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

pub use customer_postgres_repository::PostgresCustomerRepository;
pub use order_postgres_repository::PostgresOrderRepository;
pub use product_postgres_repository::PostgresProductRepository;
