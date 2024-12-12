pub mod lok;
pub mod preview_lok;
pub mod sqlite_db;

use sqlx::migrate::MigrateDatabase;
use sqlx::{Error, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://lokbuch.db";

/// Represents an error upon database operation failure.
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError,
    GeneralError(String),
    SpecificError(String),
}

pub(crate) trait Database {
    /// Type of the driver, that is a database of sqlx
    type DRV: sqlx::Database;

    /// Creates the database.
    /// Does not connect to database.
    /// Is called before every other method.
    async fn build(db_url: &str) -> Result<Self, DatabaseError>
    where
        Self: Sized;

    /// Connects to database.
    async fn connect(&mut self) -> Result<Pool<Self::DRV>, DatabaseError>;

    /// Initializes a database.
    /// Should be only called after a connection was established.
    async fn init(&self) -> Result<(), DatabaseError>;
}

pub async fn create_database() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => { println!("Database successfully created"); }
            Err(err) => { panic!("Database creation failed: {}", err) }
        }
    } else {
        println!("SQLite database already exists");
    }
}

pub async fn connect() -> Result<Pool<Sqlite>, Error> {
    SqlitePool::connect(DB_URL).await
}

pub async fn migrate(db: &Pool<Sqlite>) {
    let crate_dir = std::env::current_dir().unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(db)
        .await;

    match migration_results {
        Ok(_) => { println!("Migration success") }
        Err(err) => { panic!("error: {}", err) }
    }

    println!("migration: {:?}", migration_results);
}