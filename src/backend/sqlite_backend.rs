use crate::backend::database::lok::Lok;
use crate::backend::database::preview_lok::PreviewLok;
use crate::backend::database::sqlite_db::SQLiteDB;
use crate::backend::database::{Database, DatabaseError};
use crate::backend::Backend;
use sqlx::{Pool, Sqlite};

/// Backend implementation for a SQLite database
#[derive(Clone, Debug)]
pub struct SQLiteBackend {
    database: Pool<Sqlite>,
}

impl Default for SQLiteBackend {
    fn default() -> Self {
        SQLiteBackend {
            database: Pool::connect_lazy("").unwrap()
        }
    }
}

impl Backend for SQLiteBackend {
    async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        let mut db = SQLiteDB::build(db_url).await?;

        let conn = db.connect().await?;

        Ok(Self { database: conn })
    }

    async fn insert(&self, lok: Lok) -> u32 {
        let result = sqlx::query("INSERT INTO loks (name, address, lokmaus_name, producer, management, has_decoder, image_path) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(lok.name.clone())
            .bind(lok.address.clone())
            .bind(lok.lokmaus_name.clone())
            .bind(lok.producer.clone())
            .bind(lok.management.clone())
            .bind(lok.has_decoder.clone())
            .bind(lok.image_path.clone())
            .execute(&self.database.clone())
            .await
            .expect("Failed to add Lok to database!");

        result.last_insert_rowid() as u32
    }

    async fn get(&self, id: u32) -> Option<Lok> {
        let result = sqlx::query_as("SELECT * FROM loks WHERE id = ? LIMIT 1")
            .bind(id)
            .fetch_one(&self.database)
            .await;

        match result {
            Ok(raw_lok) => {
                Some(Lok::new_from_raw_lok_data(&raw_lok))
            }
            Err(_) => { None }
        }
    }

    async fn update(&self, id: u32, new_lok: &Lok) {
        let result = sqlx::query("UPDATE loks SET address = ?, name = ?, lokmaus_name = ?, producer = ?, management = ?, has_decoder = ?, image_path = ? WHERE id = ?;")
            .bind(new_lok.address.clone())
            .bind(new_lok.name.clone())
            .bind(new_lok.lokmaus_name.clone())
            .bind(new_lok.producer.clone())
            .bind(new_lok.management.clone())
            .bind(new_lok.has_decoder.clone())
            .bind(new_lok.image_path.clone())
            .bind(id)
            .execute(&self.database)
            .await.unwrap();

        println!("updated lok: {:?}", result)
    }

    async fn remove(&self, id: u32) {
        let result = sqlx::query("DELETE FROM loks WHERE id = ?")
            .bind(id)
            .execute(&self.database)
            .await.unwrap();

        println!("Deleted lok: {:?}", result)
    }

    async fn get_all_previews(&self) -> Vec<PreviewLok> {
        let data = sqlx::query_as("select id, address, name, lokmaus_name from loks")
            .fetch_all(&self.database)
            .await
            .unwrap();

        data.iter().map(|raw_preview| {
            PreviewLok::new_from_raw_preview_data(raw_preview)
        }).collect()
    }
}

#[cfg(test)]
mod sqlite_backend_tests {
    use super::*;
    use crate::backend::test;
    use async_std::task;

    #[test]
    fn build_works() {
        test::util::remove_test_db(12);
        let backend = task::block_on(SQLiteBackend::build("sqlite://test/test12.db"));

        assert!(backend.is_ok());
    }

    #[test]
    fn add_lok_to_new_db_works() {
        test::util::remove_test_db(13);

        let backend = task::block_on(SQLiteBackend::build("sqlite://test/test13.db")).unwrap();

        let id = task::block_on(backend.insert(test::util::get_test_lok_1()));

        println!("{}", id);

        assert_eq!(id, 1);
    }

    #[test]
    fn update_works() {
        test::util::remove_test_db(15);

        let backend = task::block_on(SQLiteBackend::build("sqlite://test/test15.db")).unwrap();

        let id = task::block_on(backend.insert(test::util::get_test_lok_1()));

        task::block_on(backend.update(id, &test::util::get_test_lok_2()))
    }

    #[test]
    fn get_lok_works() {
        test::util::remove_test_db(16);

        let backend = task::block_on(SQLiteBackend::build("sqlite://test/test16.db")).unwrap();

        let id = task::block_on(backend.insert(test::util::get_test_lok_1()));

        let lok = task::block_on(backend.get(id)).unwrap();

        assert_eq!(lok.name, String::from("TEST"));
    }

    #[test]
    fn remove_lok_works() {
        test::util::remove_test_db(17);

        let backend = task::block_on(SQLiteBackend::build("sqlite://test/test17.db")).unwrap();

        let id = task::block_on(backend.insert(test::util::get_test_lok_1()));

        task::block_on(backend.remove(id));

        let lok = task::block_on(backend.get(id));

        assert_eq!(lok, None)
    }

    #[test]
    fn get_all_previews_works() {
        test::util::remove_test_db(18);

        let backend = task::block_on(SQLiteBackend::build("sqlite://test/test18.db")).unwrap();

        let _id1 = task::block_on(backend.insert(test::util::get_test_lok_1()));
        let _id2 = task::block_on(backend.insert(test::util::get_test_lok_2()));

        let mut previews = task::block_on(backend.get_all_previews());

        assert_eq!(previews.pop().unwrap().get_name_pretty(), String::from("RRRR"));
        assert_eq!(previews.pop().unwrap().get_name_pretty(), String::from("TEST"));
    }
}