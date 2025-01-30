use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::state::State;
use crate::app::ui;
use crate::app::ui::widgets::{header, preview_widget};
use crate::app::Lokbuch;
use async_std::task;
use iced::widget::{button, container, horizontal_space, keyed_column, row, scrollable, text, text_input, vertical_space};
use iced::{Center, Element, Fill, Task};

pub struct HomePage;

impl Page for HomePage {
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
        let num_of_loks = lokbuch.lok_resource_manager.number_of_loks();
        let mut previews = lokbuch.lok_resource_manager.get_all_previews().clone();

        let header = header(format!("{} Loks vorhanden", num_of_loks));

        let input_search = text_input("Suchen...", lokbuch.state.search_input.as_str())
            .id("lok-search")
            .on_input(Message::SearchInputChanged)
            .padding(15)
            .size(ui::HEADING_TEXT_SIZE)
            .align_x(Center);


        let search_button = button(text("Suchen").size(ui::HEADING_TEXT_SIZE))
            .on_press(Message::Search)
            .padding(15);

        let add_button = button(text("Neue Lok hinzufügen").size(ui::HEADING_TEXT_SIZE))
            .on_press(Message::Add)
            .padding(15);

        let text_row = row![
                    horizontal_space()
                    .width(10),

                    text("Adresse")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    horizontal_space(),

                    text("LM-Name")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    horizontal_space(),

                    text("Bezeichnung")
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    horizontal_space(),

                    horizontal_space(),

                    horizontal_space()
                ];

        let loks = keyed_column(
            previews.into_iter().map(move |item| {
                let preview = item.clone();
                (0, iced::widget::column!(
                        button(preview_widget(preview.clone()))
                        .style(button::text)
                        .on_press_with(move || {
                            Message::ShowLok(item.clone().get_id())
                        }),
                        vertical_space()
                        .height(10))
                    .into())
            })
        ).width(Fill);

        let content = iced::widget::column![header, row![input_search, search_button], add_button, text_row, scrollable(container(loks))].align_x(Center).spacing(20).width(Fill);

        content.into()
    }
}