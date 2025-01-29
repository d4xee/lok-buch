mod frontend;
mod app;
mod backend;

use crate::app::Lokbuch;
use crate::backend::database;

fn main() -> iced::Result {
    iced::application(Lokbuch::title, Lokbuch::update, Lokbuch::view)
        .window(iced::window::Settings {
            icon: Some(iced::window::icon::from_file(frontend::ICON_PATH).unwrap()),
            ..Default::default()
        })
        .font(include_bytes!("../res/fonts/icons.ttf").as_slice())
        .centered()
        .run_with(Lokbuch::new)
}