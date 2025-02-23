#[macro_use]
extern crate rust_i18n;
mod app;

use crate::app::Lokbuch;
use app::backend::database;
use app::ui;
use rust_i18n::set_locale;

// init i18n
i18n!("res/locales", fallback="en");

/// Starting point of the application
fn main() -> iced::Result {
    set_locale("de");
    iced::application(Lokbuch::title, Lokbuch::update, Lokbuch::view)
        .subscription(Lokbuch::subscription)
        .window(iced::window::Settings {
            icon: Some(iced::window::icon::from_file(ui::ICON_PATH).unwrap()),
            exit_on_close_request: false,
            ..Default::default()
        })
        .font(include_bytes!("../res/fonts/icons.ttf").as_slice())
        .centered()
        .run_with(Lokbuch::new)
}