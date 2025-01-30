use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::state::State;
use crate::app::ui;
use crate::app::ui::widgets::page_layout;
use crate::app::Lokbuch;
use async_std::task;
use iced::widget::{button, container, horizontal_space, row, text, text_input, vertical_space};
use iced::{Center, Element, Fill, Task};

pub struct ShowPage;

impl Page for ShowPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        match message {
            Message::Add => {
                lokbuch.change_page_to(Pages::Add);
                return text_input::focus("new-lok-name");
            }

            Message::SearchInputChanged(search_input) => {
                lokbuch.state.search_input = search_input;
            }

            Message::Search => {
                println!("Search pressed!");
                //implement search by creating a search string made of address, LMname and name
            }

            Message::ShowLok(id) => {
                lokbuch.state = State {
                    selected_lok_id: Some(id),
                    ..State::default()
                };

                lokbuch.change_page_to(Pages::Show);
            }

            Message::Cancel => {
                lokbuch.change_page_to(Pages::Home);
            }

            Message::Edit(id) => {
                let lok = task::block_on(lokbuch.lok_resource_manager.get_lok(id)).expect("Lok does not exist!"); // TODO async edit

                let name_input = lok.name.clone();

                let address_input = if let Some(address) = lok.address.clone() {
                    address.to_string()
                } else { String::new() };
                let lok_maus_name_input = if let Some(lokmaus_name) = lok.lokmaus_name.clone() {
                    lokmaus_name
                } else { String::new() };
                let producer_input = if let Some(producer) = lok.producer.clone() {
                    producer
                } else { String::new() };
                let management_input = if let Some(management) = lok.management.clone() {
                    management
                } else { String::new() };

                lokbuch.state = State {
                    selected_lok_id: Some(id),
                    name_input,
                    address_input,
                    lok_maus_name_input,
                    producer_input,
                    management_input,
                    ..State::default()
                };

                lokbuch.change_page_to(Pages::Edit);
                return text_input::focus("new-lok-name");
            }

            Message::Remove(id) => {
                task::block_on(lokbuch.lok_resource_manager.remove_lok(id)); // TODO async remove

                lokbuch.change_page_to(Pages::Home);
            }

            _ => {}
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        let mut lrm = lokbuch.lok_resource_manager.clone();
        let lok = task::block_on(lrm.get_lok(lokbuch.state.selected_lok_id.unwrap())).expect("lok not found"); // TODO async show

        let left_column = iced::widget::column!(
                    text("Adresse")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_address_pretty())
                    .size(ui::HEADING_TEXT_SIZE),

                    vertical_space(),

                    text("Bezeichnung")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.name)
                    .size(ui::HEADING_TEXT_SIZE),

                    vertical_space(),

                    text("Hersteller")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_producer_pretty())
                    .size(ui::HEADING_TEXT_SIZE),
                ).spacing(10);

        let right_column = iced::widget::column![
                    text("LOKmaus-Anzeigename")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_lokmaus_name_pretty())
                    .size(ui::HEADING_TEXT_SIZE),

                    vertical_space(),
                    vertical_space(),
                    vertical_space(),
                    vertical_space(),

                    text("Bahnverwaltung")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_management_pretty())
                    .size(ui::HEADING_TEXT_SIZE)
                ].spacing(10);

        let edit_button = button(row![ui::font::edit_icon(), text("Bearbeiten")].spacing(5).align_y(Center))
            .on_press_with(move || {
                Message::Edit(lokbuch.state.selected_lok_id.clone().unwrap())
            })
            .padding(15)
            .width(Fill);

        let remove_button = button(row![ui::font::delete_icon(), text("Löschen")].spacing(5).align_y(Center))
            .on_press_with(move || {
                Message::Remove(lokbuch.state.selected_lok_id.clone().unwrap())
            })
            .style(button::danger)
            .padding(15)
            .width(Fill);

        let content = container(
            iced::widget::column![
                row![
                    horizontal_space(),
                    left_column,
                    horizontal_space(),
                    right_column,
                    horizontal_space(),
                ].width(Fill).height(Fill)
            ].width(Fill)
        );

        page_layout(format!("{}", lok.name), iced::widget::column![
            edit_button,
            remove_button,
        ], content, true)
    }
}