use crate::backend::Backend;
use crate::database::lok::Lok;
use crate::database::preview_lok::PreviewLok;
use crate::database::{Database, DatabaseError};
use sqlx::Database as Driver;
use std::collections::HashMap;

const DB_URL: &str = "sqlite://lokbuch.db";

/// The LokResourceManager is responsible for direct interaction with the data.
/// Manages the database, the cache and the preview cache.
pub struct LokResourceManager<BE: Backend> {
    backend: BE,
    cache: HashMap<u32, Lok>,
    preview_cache: Vec<PreviewLok>,
}

impl<BE: Backend> LokResourceManager<BE>
{
    pub async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        let backend = BE::build(db_url).await?;

        Ok(LokResourceManager {
            backend,
            cache: HashMap::new(),
            preview_cache: vec![],
        })
    }

    /// Adds a new lok into the database, to the cache and the preview cache.
    /// Returns the new loks id.
    pub async fn add_lok(&mut self, lok: Lok) -> u32 {
        let id = self.backend.insert(lok.clone()).await;

        self.cache.insert(id, lok.clone());
        self.preview_cache.push(lok.as_preview_lok(id as i32));

        id
    }
}

#[cfg(test)]
mod lok_resource_manager_tests {
    use super::*;
    use crate::backend::sqlite_backend::SQLiteBackend;
    use crate::backend::test;
    use async_std::task;

    #[test]
    fn build_works() {
        test::util::remove_test_db(2);
        let lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test2.db"));

        assert!(lrm.is_ok());
    }

    #[test]
    fn add_lok_on_new_db_works() {
        test::util::remove_test_db(3);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test3.db")).unwrap();

        let id = task::block_on(lrm.add_lok(Lok::new_from_raw_data("TEST".to_string(), 114141, "14TE".to_string(), "Roco".to_string(), "ÖBB".to_string())));

        println!("{}", id);

        assert_eq!(id, 1);
    }

    #[test]
    fn add_lok_to_existing_db_works() {
        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test4.db")).unwrap();

        let id = task::block_on(lrm.add_lok(Lok::new_from_raw_data("TEST".to_string(), 114141, "14TE".to_string(), "Roco".to_string(), "ÖBB".to_string())));

        println!("{}", id);

        assert!(id >= 1);
    }

    #[test]
    fn add_lok_to_db_cache_and_preview_cache_works() {
        test::util::remove_test_db(5);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test5.db")).unwrap();

        let id = task::block_on(lrm.add_lok(Lok::new_from_raw_data("TEST".to_string(), 114141, "14TE".to_string(), "Roco".to_string(), "ÖBB".to_string())));

        assert_eq!(id, 1);
        assert!(lrm.cache.len() > 0);
        assert!(lrm.preview_cache.len() > 0);
    }
}