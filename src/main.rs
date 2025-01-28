mod ui;
mod app;
mod backend;

use crate::app::{Lokbuch, SavedData};
use crate::backend::database;
use crate::backend::resource_manager::LokResourceManager;
use crate::backend::sqlite_backend::SQLiteBackend;

const DB_URL: &str = "sqlite://data/lokbuch.db";

async fn init_backend() -> SavedData {
    let lrm = LokResourceManager::<SQLiteBackend>::build(DB_URL)
        .await
        .expect("Couldn't create LokResourceManager");

    SavedData { lrm }
}

fn main() -> iced::Result {
    iced::application(Lokbuch::title, Lokbuch::update, Lokbuch::view)
        .window(iced::window::Settings {
            icon: Some(iced::window::icon::from_file(ui::ICON_PATH).unwrap()),
            ..Default::default()
        })
        .font(include_bytes!("../res/fonts/icons.ttf").as_slice())
        .centered()
        .run_with(Lokbuch::new)
}