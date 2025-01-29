use crate::app::message::Message;
use crate::app::page::Page;
use crate::app::Lokbuch;
use iced::widget::text;
use iced::{Element, Task};

pub struct SettingsPage;

impl Page for SettingsPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        text("settings").into()
    }
}