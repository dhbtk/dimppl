use crate::directories::project_dirs;
use diesel::sqlite::Sqlite;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn db_connect() -> SqliteConnection {
    loop {
        let connection = SqliteConnection::establish(database_path().as_ref());
        match connection {
            Ok(conn) => return conn,
            Err(e) => {
                tracing::error!("Error connecting to database: {}", e);
                sleep(Duration::from_millis(100));
            }
        }
    }
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
