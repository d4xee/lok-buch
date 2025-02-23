use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::ui;
use crate::app::Lokbuch;
use iced::widget::{center, text};
use iced::{Element, Task};
use rust_i18n::set_locale;

pub struct LoadingPage;

impl Page for LoadingPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        lokbuch.moving_icon_frames = ui::moving_icon_frames();

        match message {
            Message::Loaded(persistent_data) => {
                lokbuch.lok_resource_manager = persistent_data.get_lok_resource_manager();
                lokbuch.settings = persistent_data.get_settings();

                set_locale(lokbuch.settings.language.as_str());

                lokbuch.change_page_to(Pages::Home);
            }
            _ => {}
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        center(iced::widget::column![
                    iced_gif::Gif::new(&lokbuch.moving_icon_frames),
                    text(t!("loading.loading")).size(50)])
            .into()
    }
}