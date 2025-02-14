use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::ui::widgets::page_layout;
use crate::app::Lokbuch;
use iced::widget::{text, Container};
use iced::{Element, Task};

pub struct SettingsPage;

impl Page for SettingsPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        match message {
            Message::Cancel => {
                lokbuch.change_page_to(Pages::Home)
            }

            _ => {}
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        page_layout(t!("settings.settings").to_string(), iced::widget::Column::new(), Container::new(text("test")), true)
    }
}