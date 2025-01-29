use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::Lokbuch;
use async_std::task;
use iced::{Element, Task};

pub struct AddPage;

impl Page for AddPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        match message {
            Message::AddNewLok => {
                if let Some(error_task) = lokbuch.state.validate().err() {
                    return error_task;
                }

                let new_lok = lokbuch.state.get_lok_from_current_state();

                task::block_on(lokbuch.lok_resource_manager.add_lok(new_lok.clone()));

                lokbuch.state.clear();
                lokbuch.change_page_to(Pages::Home);
            }
            Message::Cancel => {
                lokbuch.state.clear();
                lokbuch.change_page_to(Pages::Home);
            }
            _ => {
                lokbuch.state.update(message);
            }
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        lokbuch.lok_data_input_mask(String::from("Hinzufügen"), Message::AddNewLok)
    }
}