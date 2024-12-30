use crate::directories::project_dirs;
use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::Sqlite;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
use std::ops::Deref;
use std::sync::LazyLock;
use std::time::Duration;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>,
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            if let Some(d) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static POOL: LazyLock<Pool<ConnectionManager<SqliteConnection>>> = LazyLock::new(|| {
    let manager = ConnectionManager::<SqliteConnection>::new(database_path());
    Pool::builder()
        .test_on_check_out(true)
        .connection_customizer(Box::new(ConnectionOptions {
            enable_wal: true,
            enable_foreign_keys: true,
            busy_timeout: Some(Duration::from_secs(30)),
        }))
        .build(manager)
        .expect("Failed to create connection pool")
});

pub fn db_connect() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    POOL.deref().get().unwrap()
}

pub fn migrate_database(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn prepare_database() {
    let mut conn = db_connect();
    let _ = migrate_database(&mut conn);
    // TODO reset db in case the migration fails
}

pub fn database_path() -> String {
    let directories = project_dirs();
    let path = directories.data_dir().to_path_buf().join("database.db");
    path.to_str().unwrap().into()
}
