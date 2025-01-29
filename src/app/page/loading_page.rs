use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::Lokbuch;
use crate::frontend;
use iced::widget::{center, text};
use iced::{Element, Task};

pub struct LoadingPage;

impl Page for LoadingPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        lokbuch.moving_icon_frames = frontend::moving_icon_frames();

        match message {
            Message::Loaded(saved_data) => {
                lokbuch.lok_resource_manager = saved_data.lrm.clone();
                lokbuch.change_page_to(Pages::Home);
            }
            _ => {}
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        center(iced::widget::column![
                    iced_gif::Gif::new(&lokbuch.moving_icon_frames),
                    text("Laden...").size(50)])
            .into()
    }
}