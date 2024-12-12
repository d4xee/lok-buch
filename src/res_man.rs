use crate::database::lok::Lok;
use crate::database::preview_lok::PreviewLok;
use crate::database::sqlite_db::SQLiteDB;
use crate::database::{Database, DatabaseError};
use sqlx::Database as Driver;
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

const DB_URL: &str = "sqlite://lokbuch.db";

/// The LokResourceManager is responsible for direct interaction with the data.
/// Manages the database, the cache and the preview cache.
pub struct LokResourceManager<DB: Database, DRV: Driver> {
    database: DB,
    connection: Pool<DRV>,
    cache: HashMap<u32, Lok>,
    preview_cache: Vec<PreviewLok>,
}

impl<DB: Database<DRV=Sqlite>> LokResourceManager<DB, Sqlite>
{
    pub async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        let mut db = DB::build(db_url).await?;

        let conn = db.connect().await?;

        Ok(LokResourceManager {
            database: db,
            connection: conn,
            cache: HashMap::new(),
            preview_cache: vec![],
        })
    }

    /// Adds a new lok into the database, to the cache and the preview cache.
    /// Returns the new loks id.
    pub async fn add_lok(&mut self, lok: Lok) -> u32 {
        let result = sqlx::query("INSERT INTO loks (name, address, lokmaus_name, producer, management) VALUES (?, ?, ?, ?, ?)")
            .bind(lok.name.clone())
            .bind(lok.address.clone())
            .bind(lok.lokmaus_name.clone())
            .bind(lok.producer.clone())
            .bind(lok.management.clone())
            .execute(&self.connection.clone())
            .await
            .expect("Failed to add Lok to database!");

        let id = result.last_insert_rowid() as u32;

        self.cache.insert(id, lok.clone());
        self.preview_cache.push(lok.as_preview_lok(id as i32));

        id
    }
}

#[cfg(test)]
mod lok_resource_manager_tests {
    use super::*;
    use crate::test;
    use async_std::task;

    #[test]
    fn build_works() {
        test::util::remove_test_db(2);
        let lrm = task::block_on(LokResourceManager::<SQLiteDB, Sqlite>::build("sqlite://test/test2.db"));

        assert!(lrm.is_ok());
    }

    #[test]
    fn add_lok_on_new_db_works() {
        test::util::remove_test_db(3);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteDB, Sqlite>::build("sqlite://test/test3.db")).unwrap();

        let id = task::block_on(lrm.add_lok(Lok::new_from_raw_data("TEST".to_string(), 114141, "14TE".to_string(), "Roco".to_string(), "ÖBB".to_string())));

        println!("{}", id);

        assert_eq!(id, 1);
    }

    #[test]
    fn add_lok_to_existing_db_works() {
        let mut lrm = task::block_on(LokResourceManager::<SQLiteDB, Sqlite>::build("sqlite://test/test4.db")).unwrap();

        let id = task::block_on(lrm.add_lok(Lok::new_from_raw_data("TEST".to_string(), 114141, "14TE".to_string(), "Roco".to_string(), "ÖBB".to_string())));

        println!("{}", id);

        assert!(id >= 1);
    }

    #[test]
    fn add_lok_to_db_cache_and_preview_cache_works() {
        test::util::remove_test_db(5);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteDB, Sqlite>::build("sqlite://test/test5.db")).unwrap();

        let id = task::block_on(lrm.add_lok(Lok::new_from_raw_data("TEST".to_string(), 114141, "14TE".to_string(), "Roco".to_string(), "ÖBB".to_string())));

        assert_eq!(id, 1);
        assert!(lrm.cache.len() > 0);
        assert!(lrm.preview_cache.len() > 0);
    }
}