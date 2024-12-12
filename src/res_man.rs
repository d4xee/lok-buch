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

impl<DB: Database<DRV=DRV>, DRV: Driver> LokResourceManager<DB, DRV> {
    pub async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        let mut db = DB::build(db_url).await?;

        let conn: Pool<DRV> = db.connect().await?;

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
        let lrm = task::block_on(LokResourceManager::<SQLiteDB, Sqlite>::build("sqlite://test/test.db"));

        assert!(lrm.is_ok());
    }
}