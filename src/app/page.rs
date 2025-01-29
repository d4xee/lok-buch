pub mod loading_page;
pub mod home_page;
pub mod add_page;
pub mod edit_page;
pub mod show_page;
pub mod settings_page;

use crate::app::message::Message;
use crate::app::page::add_page::AddPage;
use crate::app::page::edit_page::EditPage;
use crate::app::page::home_page::HomePage;
use crate::app::page::loading_page::LoadingPage;
use crate::app::page::settings_page::SettingsPage;
use crate::app::page::show_page::ShowPage;
use crate::app::Lokbuch;
use iced::{Element, Task};

pub trait Page {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message>;
    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message>;
}

pub enum Pages {
    Add,
    Edit,
    Show,
    Settings,
    Home,
    Loading,
}

impl Pages {
    pub fn as_page_struct(&self) -> Box<dyn Page> {
        match self {
            Pages::Add => { Box::new(AddPage) }
            Pages::Edit => { Box::new(EditPage) }
            Pages::Show => { Box::new(ShowPage) }
            Pages::Settings => { Box::new(SettingsPage) }
            Pages::Home => { Box::new(HomePage) }
            Pages::Loading => { Box::new(LoadingPage) }
        }
    }
}