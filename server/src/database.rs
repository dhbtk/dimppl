use std::env;

use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::schema::users::dsl::users;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type AsyncConnection<'a> =
    bb8::PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn create_database_pool() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL variable set!");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url.clone());
    let pool = bb8::Pool::builder().build_unchecked(config);
    let mut conn = PgConnection::establish(db_url.as_ref()).unwrap();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");
    if env::var("DIMPPL_TEST").is_ok() {
        diesel::delete(users)
            .execute(&mut conn)
            .expect("Error clearing database");
    }
    pool
}

#[cfg(test)]
pub fn establish_pg_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL variable set!");
    PgConnection::establish(db_url.as_ref()).unwrap()
}
