pub mod lok;
pub mod preview_lok;

use sqlx::migrate::MigrateDatabase;
use sqlx::{Error, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://lokbuch.db";

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