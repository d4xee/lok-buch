use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::ui::widgets::lok_data_input_mask;
use crate::app::Lokbuch;
use async_std::task;
use iced::{Element, Task};
use rfd::FileDialog;
use std::fs;

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
            Message::SelectImageFile => {
                let file = FileDialog::new()
                    .add_filter(t!("add.image_files"), &["png", "jpg", "jpeg"])
                    .pick_file();

                // if a file was selected, it is copied to user data and timestamped
                if let Some(image_file) = file {
                    println!("Selected image file: {:?}", image_file);

                    let image_type = image_file.extension().unwrap().to_str().unwrap().to_ascii_lowercase();
                    let datetime = chrono::Local::now().format("%Y-%m-%d_%H%M%S").to_string();

                    let image_path = format!("./data/images/{datetime}.{image_type}");

                    fs::copy(&image_file, &image_path).unwrap();

                    lokbuch.state.image_path_input = image_path;
                }
            }
            Message::Cancel => {
                // if exists, removes the image file
                fs::remove_file(lokbuch.state.image_path_input.clone()).ok();

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
        lok_data_input_mask(lokbuch, t!("add.add").to_string(), Message::AddNewLok)
    }
}