mod app;

use crate::app::Lokbuch;
use app::backend::database;
use app::ui;

/// Starting point of the application
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