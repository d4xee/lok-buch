use crate::database::sqlite_db::SQLiteDB;
use crate::database::{Database, DatabaseError};
use sqlx::Database as Driver;
use sqlx::{Pool, Sqlite};

const DB_URL: &str = "sqlite://lokbuch.db";

/// The LokResourceManager is responsible for direct interaction with the database
pub struct LokResourceManager<DB: Database, DRV: Driver> {
    database: DB,
    connection: Pool<DRV>,
}

impl LokResourceManager<SQLiteDB, Sqlite> {
    pub async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        let mut db = SQLiteDB::build(db_url).await?;

        let conn = db.connect().await?;

        db.init().await?;

        Ok(LokResourceManager { database: db, connection: conn })
    }
}

#[cfg(test)]
mod lok_resource_manager_tests {
    use super::*;
    use async_std::task;

    #[test]
    fn build_works() {
        let lrm = task::block_on(LokResourceManager::build("sqlite://test/test.db"));

        assert!(lrm.is_ok());
    }
}