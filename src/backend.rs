use crate::backend::database::lok::Lok;
use crate::backend::database::preview_lok::PreviewLok;
use crate::backend::database::DatabaseError;

pub mod database;
pub mod resource_manager;
mod test;
mod sqlite_backend;

/// The backend is responsible for the direct communication with the database.
/// It encapsulates the concrete SQL statements.
pub trait Backend: Sized + Clone {
    async fn build(db_url: &str) -> Result<Self, DatabaseError>;

    async fn insert(&self, lok: Lok) -> u32;

    async fn get(&self, id: u32) -> Option<Lok>;

    async fn update(&self, id: u32, new_lok: &Lok);

    async fn remove(&self, id: u32);

    async fn get_all_previews(&self) -> Vec<PreviewLok>;
}