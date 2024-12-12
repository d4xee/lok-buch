pub mod database;
mod ui;
mod app;

mod res_man;
mod test;

use crate::app::{Lokbuch, SavedData};
use crate::database::lok::Lok;
use crate::database::preview_lok::PreviewLok;
use sqlx::{Pool, Sqlite};

async fn add_new_lok(db: Pool<Sqlite>, lok: Lok) -> i32 {
    lok.save(&db).await
}

async fn delete_lok_by_id(db: Pool<Sqlite>, id: i32) {
    Lok::delete_lok_by_id(&db, id).await;
}

async fn update_lok_by_id(db: Pool<Sqlite>, old_lok_id: i32, new_lok: Lok) {
    Lok::update_lok_by_id(&db, old_lok_id, new_lok).await;
}

async fn init_database() -> SavedData {
    database::create_database().await;

    let db = database::connect().await.unwrap();

    database::migrate(&db).await;

    let mut loks = database::preview_lok::get_all_previews(&db).await;

    loks.sort();

    SavedData { db: db.clone(), loks_preview: loks }
}

async fn get_updated_lok_list(db: Pool<Sqlite>) -> Vec<PreviewLok> {
    let mut loks = database::preview_lok::get_all_previews(&db).await;
    loks.sort();

    loks
}

fn main() -> iced::Result {
    iced::application(Lokbuch::title, Lokbuch::update, Lokbuch::view)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .centered()
        .run_with(Lokbuch::new)
}