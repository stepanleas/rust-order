use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/migrations");

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn configure(database_url: String) -> anyhow::Result<DbPool> {
    let mut connection = PgConnection::establish(database_url.as_str())?;
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let manager = ConnectionManager::<PgConnection>::new(database_url.as_str());
    Ok(Pool::builder().build(manager)?)
}
