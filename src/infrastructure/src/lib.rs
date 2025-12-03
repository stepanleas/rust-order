pub mod config;
pub mod entities;
pub mod enums;
pub mod outbox;
pub mod postgres_customer_repository;
mod postgres_customer_repository_tests;
pub mod postgres_order_repository;
mod postgres_order_repository_tests;
pub mod postgres_product_repository;
mod postgres_product_repository_tests;
mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
