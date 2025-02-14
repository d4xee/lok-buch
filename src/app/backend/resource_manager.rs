use crate::app::backend::Backend;
use crate::database::lok::Lok;
use crate::database::preview_lok::PreviewLok;
use crate::database::DatabaseError;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

/// The LokResourceManager is responsible for direct interaction with the data.
/// Manages the database, the cache and the preview cache.
#[derive(Clone)]
pub struct LokResourceManager<BE: Backend> {
    backend: BE,
    cache: HashMap<u32, Lok>,
    preview_cache: Vec<PreviewLok>,
    search_results: Vec<PreviewLok>,
}

impl<BE: Backend> LokResourceManager<BE>
{
    /// Builds a LokResourceManager (LRM) for a certain backend.
    pub async fn build(db_url: &str) -> Result<Self, DatabaseError> {
        let backend = BE::build(db_url).await?;

        let mut preview_cache = backend.get_all_previews().await;
        preview_cache.sort_unstable();

        Ok(LokResourceManager {
            backend: backend.clone(),
            cache: HashMap::new(),
            preview_cache,
            search_results: Vec::new(),
        })
    }

    /// Adds a new lok into the database, to the cache and the preview cache.
    /// Returns the new loks id.
    pub async fn add_lok(&mut self, lok: Lok) -> u32 {
        let id = self.backend.insert(lok.clone()).await;

        self.cache.insert(id, lok.clone());
        self.preview_cache.push(lok.as_preview_lok(id));
        self.preview_cache.sort();

        id
    }

    /// Returns a ```Some(Lok)``` if the id exists.
    /// Returns ```None``` otherwise.
    pub async fn get_lok(&mut self, id: u32) -> Option<Lok> {
        if let Some(lok) = self.cache.get(&id) {
            return Some(lok.clone());
        } else {
            if let Some(lok) = self.backend.get(id).await {
                self.cache.insert(id, lok.clone());

                return Some(lok.clone());
            }
        }
        None
    }

    /// Removes a lok from the caches and the database.
    pub async fn remove_lok(&mut self, id: u32) {
        self.cache.remove(&id);
        let be_remove = self.backend.remove(id);

        let index = self.find_preview_index(id);

        if let Some(index) = index {
            let _ = self.preview_cache.remove(index as usize);
            self.preview_cache.sort();
        }


        be_remove.await;
    }

    /// Updates a lok with the new data from new_lok.
    pub async fn update_lok(&mut self, id: u32, new_lok: Lok) {
        if self.cache.get(&id).is_some() {
            self.cache.insert(id, new_lok.clone());
        } else {
            self.cache.insert(id, new_lok.clone());
        }

        let be_update = self.backend.update(id, &new_lok);

        let index = self.find_preview_index(id);

        if let Some(index) = index {
            let _ = self.preview_cache.remove(index as usize);

            self.preview_cache.push(new_lok.clone().as_preview_lok(id));
            self.preview_cache.sort();
        }

        be_update.await;
    }

    /// Returns all previews at once
    pub fn get_all_previews(&self) -> Vec<PreviewLok> {
        self.preview_cache.clone()
    }

    /// Finds the index of a lok in the preview list, which is in different order than the database.
    /// If the id does not exist, ```None``` is returned
    fn find_preview_index(&self, id: u32) -> Option<u32> {
        let mut index = 0;
        for preview in self.preview_cache.iter() {
            if preview.get_id() == id {
                return Some(index);
            }
            index += 1;
        }

        None
    }

    /// Returns the number of saved loks.
    pub fn number_of_loks(&self) -> u32 {
        self.preview_cache.len() as u32
    }

    /// Stores every PreviewLok that matches with the given search string
    pub fn search_and_store_previews_containing(&mut self, search: String) {
        self.search_results = self.preview_cache
            .iter()
            .filter(|preview_lok| { preview_lok.get_search_string().contains(search.as_str()) })
            .map(|preview_lok| { preview_lok.clone() })
            .collect::<Vec<PreviewLok>>()
            .into();
    }

    /// Returns every PreviewLok that matches with the previously given search string
    pub fn get_search_results(&self) -> Vec<PreviewLok> {
        self.search_results.clone()
    }
}

impl<BE: Backend> Default for LokResourceManager<BE> {
    fn default() -> Self {
        LokResourceManager {
            backend: BE::default(),
            cache: HashMap::default(),
            preview_cache: Vec::default(),
            search_results: Vec::default(),
        }
    }
}


impl<BE: Backend> Debug for LokResourceManager<BE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LokResourceManager")
            .field("backend", &self.backend)
            .field("cache", &self.cache)
            .field("preview_cache", &self.preview_cache)
            .finish()
    }
}

#[cfg(test)]
mod lok_resource_manager_tests {
    use super::*;
    use crate::app::backend::sqlite_backend::SQLiteBackend;
    use crate::app::backend::test;
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

        let id = task::block_on(lrm.add_lok(test::util::get_test_lok_1()));

        println!("{}", id);

        assert_eq!(id, 1);
    }

    #[test]
    fn add_lok_to_db_cache_and_preview_cache_works() {
        test::util::remove_test_db(5);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test5.db")).unwrap();

        let id = task::block_on(lrm.add_lok(test::util::get_test_lok_1()));

        assert_eq!(id, 1);
        assert!(lrm.cache.len() > 0);
        assert!(lrm.preview_cache.len() > 0);
    }

    #[test]
    fn get_lok_works() {
        test::util::remove_test_db(6);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test6.db")).unwrap();

        let result = task::block_on(lrm.get_lok(1));

        assert!(result.is_none());

        let _id = task::block_on(lrm.add_lok(test::util::get_test_lok_1()));

        let result = task::block_on(lrm.get_lok(1));

        assert!(result.is_some());
    }

    #[test]
    fn remove_works() {
        test::util::remove_test_db(7);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test7.db")).unwrap();

        let id = task::block_on(lrm.add_lok(test::util::get_test_lok_1()));

        let result = task::block_on(lrm.get_lok(id));

        assert!(result.is_some());

        task::block_on(lrm.remove_lok(id));

        let result = task::block_on(lrm.get_lok(id));

        assert!(result.is_none());
    }

    #[test]
    fn update_works() {
        test::util::remove_test_db(8);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test8.db")).unwrap();

        let id = task::block_on(lrm.add_lok(test::util::get_test_lok_1()));

        task::block_on(lrm.update_lok(id, test::util::get_test_lok_2()));

        let result = task::block_on(lrm.get_lok(id)).unwrap();

        let test_lok2 = test::util::get_test_lok_2();

        assert_eq!(result.name, test_lok2.name);
        assert_eq!(result.address.unwrap(), test_lok2.address.unwrap());
        assert_eq!(result.lokmaus_name.unwrap(), test_lok2.lokmaus_name.unwrap());
        assert_eq!(result.producer.unwrap(), test_lok2.producer.unwrap());
        assert_eq!(result.management.unwrap(), test_lok2.management.unwrap());
        assert_eq!(result.has_decoder, test_lok2.has_decoder);
        assert_eq!(result.image_path.unwrap(), test_lok2.image_path.unwrap());
    }

    #[test]
    fn get_all_previews_works() {
        test::util::remove_test_db(9);

        let mut lrm = task::block_on(LokResourceManager::<SQLiteBackend>::build("sqlite://test/test9.db")).unwrap();

        let _id1 = task::block_on(lrm.add_lok(test::util::get_test_lok_1()));
        let _id2 = task::block_on(lrm.add_lok(test::util::get_test_lok_2()));

        let mut previews = lrm.get_all_previews();

        assert_eq!(previews.pop().unwrap().get_name_pretty(), String::from("TEST"));
        assert_eq!(previews.pop().unwrap().get_name_pretty(), String::from("RRRR"));
    }
}