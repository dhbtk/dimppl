use std::env;
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use diesel::{Connection, PgConnection, RunQueryDsl};
use tokio::task::{block_in_place, futures};
use crate::schema::users::dsl::users;

pub fn create_database_pool() -> Pool {
    let manager = Manager::new(
        env::var("DATABASE_URL").expect("No DATABASE_URL variable set!"),
        Runtime::Tokio1
    );
    let pool = Pool::builder(manager)
        .max_size(8)
        .build()
        .unwrap();
    if std::env::var("DIMPPL_TEST").is_ok() {
        let mut conn = PgConnection::establish(&*env::var("DATABASE_URL").unwrap()).unwrap();
        diesel::delete(users).execute(&mut conn).expect("Error clearing database");
    }
    pool
}
