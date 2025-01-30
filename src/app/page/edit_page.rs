use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::ui::widgets::lok_data_input_mask;
use crate::app::Lokbuch;
use async_std::task;
use iced::{Element, Task};

pub struct EditPage;

impl Page for EditPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        match message {
            Message::EditLok => {
                if let Some(error_task) = lokbuch.state.validate().err() {
                    return error_task;
                }

                let new_lok = lokbuch.state.get_lok_from_current_state();

                let old_lok_id = lokbuch.state.selected_lok_id.clone().unwrap();

                task::block_on(lokbuch.lok_resource_manager.update_lok(old_lok_id, new_lok));

                lokbuch.state.clear();
                lokbuch.change_page_to(Pages::Home);
            }
            Message::Cancel => {
                lokbuch.state.clear();
                lokbuch.change_page_to(Pages::Home);
            }
            _ => { lokbuch.state.update(message); }
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        lok_data_input_mask(lokbuch, String::from("Bearbeiten"), Message::EditLok)
    }
}