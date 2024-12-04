mod database;
mod ui;
mod app;

use crate::app::{Lokbuch, SavedData};
use crate::database::lok::Lok;
use sqlx::{Pool, Sqlite};


async fn add_new_lok(db: Pool<Sqlite>, lok: Lok) -> Result<(), ()> {
    lok.save(&db).await;

    Ok(())
}

async fn delete_lok(db: Pool<Sqlite>, lok: Lok) {
    lok.delete(&db).await;
}

async fn update_lok(db: Pool<Sqlite>, old_lok: Lok, new_lok: Lok) {
    old_lok.update(&db, new_lok).await;
}

async fn init_database() -> SavedData {
    database::create_database().await;

    let db = database::connect().await.unwrap();

    database::migrate(&db).await;

    let mut loks = database::lok::get_all_loks(&db).await;

    loks.sort();

    SavedData { db: db.clone(), loks }
}

async fn get_updated_lok_list(db: Pool<Sqlite>) -> Vec<Lok> {
    let mut loks = database::lok::get_all_loks(&db).await;
    loks.sort();

    loks
}

fn main() -> iced::Result {
    iced::application(Lokbuch::title, Lokbuch::update, Lokbuch::view)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .centered()
        .run_with(Lokbuch::new)
}