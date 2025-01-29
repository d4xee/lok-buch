use crate::backend::resource_manager::LokResourceManager;
use crate::backend::sqlite_backend::SQLiteBackend;

#[derive(Clone, Debug)]
pub struct StoredData {
    pub lrm: LokResourceManager<SQLiteBackend>,
}

impl StoredData {
    pub async fn init_backend(db_url: &str) -> StoredData {
        let lrm = LokResourceManager::<SQLiteBackend>::build(db_url)
            .await
            .expect("Couldn't create LokResourceManager");

        StoredData { lrm }
    }
}
