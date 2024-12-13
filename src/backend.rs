use crate::backend::database::lok::Lok;
use crate::backend::database::preview_lok::PreviewLok;
use crate::backend::database::{Database, DatabaseError};
use sqlx::Database as Driver;

pub mod database;
pub mod resource_manager;
mod test;
mod sqlite_backend;

/// The backend is responsible for the direct communication with the database.
/// It encapsulates the concrete SQL statements.
trait Backend: Sized {
    async fn build(db_url: &str) -> Result<Self, DatabaseError>;

    async fn insert(&self, lok: Lok) -> u32;

    async fn get(&self, id: u32) -> Result<Lok, DatabaseError>;

    async fn update(&self, id: u32, new_lok: Lok);

    async fn remove(&self, id: u32);

    async fn get_all_previews(&self) -> Vec<PreviewLok>;
}