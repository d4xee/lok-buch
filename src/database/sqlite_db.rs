use crate::database::{Database, DatabaseError};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Sqlite, SqlitePool};

pub struct SQLiteDB {
    connection: Pool<Sqlite>,
    db_url: String,
}

impl Database for SQLiteDB {
    async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            println!("Creating database {}", db_url);
            match Sqlite::create_database(db_url).await {
                Ok(_) => {
                    Ok(SQLiteDB {
                        connection: Pool::connect_lazy(db_url).unwrap(),
                        db_url: db_url.to_owned(),
                    })
                }
                Err(err) => {
                    Err(DatabaseError::GeneralError(format!("Failed to create SQLite database: {}", err)))
                }
            }
        } else {
            println!("SQLite database already exists");

            Ok(SQLiteDB {
                connection: Pool::connect_lazy(db_url).unwrap(),
                db_url: db_url.to_owned(),
            })
        }
    }

    async fn connect(&mut self) -> Result<Pool<Sqlite>, DatabaseError>
    {
        match SqlitePool::connect(self.db_url.as_str()).await {
            Ok(connection) => {
                self.connection = connection.clone();
                Ok(connection)
            }
            Err(_) => {
                Err(DatabaseError::ConnectionError)
            }
        }
    }

    async fn init(&self) -> Result<(), DatabaseError> {
        // sqlite migrations
        let crate_dir = std::env::current_dir().unwrap();
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");

        let migration_results = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&self.connection)
            .await;

        match migration_results {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                Err(DatabaseError::SpecificError(format!("Error running migrations: {}", err.to_string())))
            }
        }
    }
}

#[cfg(test)]
mod sqlite_db_tests {
    use super::*;
    use async_std::task;

    fn remove_test_db() {
        match std::fs::remove_file("test/test.db") {
            Ok(_) => { println!("Removed test db"); }
            Err(_) => { println!("Failed to remove test db"); }
        }
    }

    fn build_db() -> SQLiteDB {
        remove_test_db();
        task::block_on(SQLiteDB::build("sqlite://test/test.db")).unwrap()
    }

    #[test]
    fn build_db_test() {
        remove_test_db();
        let result = task::block_on(SQLiteDB::build("sqlite://test/test.db"));

        assert!(result.is_ok());
    }

    #[test]
    fn connect_to_db_test() {
        let mut db = build_db();

        let result = task::block_on(db.connect());

        assert!(result.is_ok())
    }

    #[test]
    fn init_db_test() {
        let mut db = build_db();

        let _ = task::block_on(db.connect());

        let result = task::block_on(db.init());

        assert!(result.is_ok());
    }
}