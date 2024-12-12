use crate::database::{Database, DatabaseError};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Sqlite, SqlitePool};

/// SQLite implementation for the database.
pub struct SQLiteDB {
    connection: Pool<Sqlite>,
    db_url: String,
}

impl SQLiteDB {
    async fn migrate(connection: Pool<Sqlite>) -> Result<(), DatabaseError> {
        // sqlite migrations
        let crate_dir = std::env::current_dir().unwrap();
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");

        let migration_results = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&connection)
            .await;

        match migration_results {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                Err(DatabaseError::SpecificError(format!("Error running migrations: {}", err)))
            }
        }
    }
}

impl Database for SQLiteDB {
    type DRV = Sqlite;

    async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            println!("Creating database {}", db_url);
            match Sqlite::create_database(db_url).await {
                Ok(_) => {
                    let connection = Pool::connect(db_url).await;

                    if connection.is_ok() {
                        let connection = connection.unwrap();
                        SQLiteDB::migrate(connection.clone()).await?;

                        Ok(SQLiteDB {
                            connection: connection.clone(),
                            db_url: db_url.to_owned(),
                        })
                    } else {
                        Err(DatabaseError::ConnectionError)
                    }
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
}

#[cfg(test)]
mod sqlite_db_tests {
    use super::*;
    use crate::test;
    use async_std::task;
    use std::time::Duration;


    #[test]
    fn build_db_test() {
        test::util::remove_test_db(0);
        std::thread::sleep(Duration::from_millis(test::util::AFTER_DELETE_WAIT_TIME));
        let result = task::block_on(SQLiteDB::build("sqlite://test/test0.db"));

        assert!(result.is_ok());
    }

    #[test]
    fn connect_to_db_test() {
        let mut db = test::util::build_db(1);

        let result = task::block_on(db.connect());

        assert!(result.is_ok())
    }
}