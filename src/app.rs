mod state;
mod stored_data;
pub(crate) mod message;

use crate::app::message::Message;
use crate::app::state::State;
use crate::app::stored_data::StoredData;
use crate::backend::resource_manager::LokResourceManager;
use crate::backend::sqlite_backend::SQLiteBackend;
use crate::frontend;
use crate::frontend::widgets::{header, preview_widget};
use async_std::task;
use iced::widget::{button, center, checkbox, column, container, horizontal_space, image, keyed_column, row, scrollable, text, text_input, vertical_space};
use iced::{Center, ContentFit, Element, Fill, Left, Task};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DB_URL: &str = "sqlite://data/lokbuch.db";

pub struct Lokbuch {
    page: Page,
    state: State,
    lok_resource_manager: LokResourceManager<SQLiteBackend>,
    moving_icon_frames: iced_gif::Frames,
}

pub enum Page {
    Loading,
    Home,
    Add,
    Edit,
    Show,
    Settings,
}

impl Lokbuch {
    pub(crate) fn new() -> (Self, Task<Message>) {
        (Lokbuch {
            page: Page::Loading,
            state: State::default(),
            lok_resource_manager: LokResourceManager::default(),
            moving_icon_frames: frontend::moving_icon_frames(),
        },
         Task::perform(StoredData::init_backend(DB_URL), Message::Loaded))
    }

    pub(crate) fn title(&self) -> String {
        format!("Lokbuch v{}", VERSION)
    }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        match self.page {
            Page::Loading => {
                self.moving_icon_frames = frontend::moving_icon_frames();

                match message {
                    Message::Loaded(saved_data) => {
                        self.lok_resource_manager = saved_data.lrm.clone();
                        self.page = Page::Home;
                    }
                    _ => {}
                }
                Task::none()
            }

            Page::Home | Page::Show => {
                match message {
                    Message::Add => {
                        self.page = Page::Add;
                        return text_input::focus("new-lok-name");
                    }

                    Message::SearchInputChanged(search_input) => {
                        self.state.search_input = search_input;
                    }

                    Message::Search => {
                        println!("Search pressed!");
                        //implement search by creating a search string made of address, LMname and name
                    }

                    Message::ShowLok(id) => {
                        self.state = State {
                            selected_lok_id: Some(id),
                            ..State::default()
                        };

                        self.page = Page::Show;
                    }

                    Message::Cancel => {
                        self.page = Page::Home;
                    }

                    Message::Edit(id) => {
                        let lok = task::block_on(self.lok_resource_manager.get_lok(id)).expect("Lok does not exist!"); // TODO async edit

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

                        self.state = State {
                            selected_lok_id: Some(id),
                            name_input,
                            address_input,
                            lok_maus_name_input,
                            producer_input,
                            management_input,
                            ..State::default()
                        };

                        self.page = Page::Edit;
                        return text_input::focus("new-lok-name");
                    }

                    Message::Remove(id) => {
                        task::block_on(self.lok_resource_manager.remove_lok(id)); // TODO async remove

                        self.page = Page::Home;
                    }

                    _ => {}
                }
                Task::none()
            }

            Page::Add => {
                match message {
                    Message::AddNewLok => {
                        if let Some(error_task) = self.state.validate().err() {
                            return error_task;
                        }

                        let new_lok = self.state.get_lok_from_current_state();

                        task::block_on(self.lok_resource_manager.add_lok(new_lok.clone()));

                        self.state.clear();
                        self.page = Page::Home;
                    }
                    Message::Cancel => {
                        self.state.clear();
                        self.page = Page::Home;
                    }
                    _ => {
                        self.state.update(message);
                    }
                }
                Task::none()
            }

            Page::Edit => {
                match message {
                    Message::EditLok => {
                        if let Some(error_task) = self.state.validate().err() {
                            return error_task;
                        }

                        let new_lok = self.state.get_lok_from_current_state();

                        let old_lok_id = self.state.selected_lok_id.clone().unwrap();

                        task::block_on(self.lok_resource_manager.update_lok(old_lok_id, new_lok));

                        self.state.clear();
                        self.page = Page::Home;
                    }
                    Message::Cancel => {
                        self.state.clear();
                        self.page = Page::Home;
                    }
                    _ => { self.state.update(message); }
                }
                Task::none()
            }

            Page::Settings => { Task::none() }
        }
    }

    pub(crate) fn view(&self) -> Element<'_, Message> {
        match self.page {
            Page::Loading => {
                center(column![
                    iced_gif::Gif::new(&self.moving_icon_frames),
                    text("Laden...").size(50)])
                    .into()
            }

            Page::Add => {
                self.lok_data_input_mask(String::from("Hinzufügen"), Message::AddNewLok)
            }

            Page::Edit => {
                self.lok_data_input_mask(String::from("Bearbeiten"), Message::EditLok)
            }

            Page::Home => {
                let num_of_loks = self.lok_resource_manager.number_of_loks();
                let mut previews = self.lok_resource_manager.get_all_previews().clone();

                let header = header(format!("{} Loks vorhanden", num_of_loks));

                let input_search = text_input("Suchen...", self.state.search_input.as_str())
                    .id("lok-search")
                    .on_input(Message::SearchInputChanged)
                    .padding(15)
                    .size(frontend::HEADING_TEXT_SIZE)
                    .align_x(Center);


                let search_button = button(text("Suchen").size(frontend::HEADING_TEXT_SIZE))
                    .on_press(Message::Search)
                    .padding(15);

                let add_button = button(text("Neue Lok hinzufügen").size(frontend::HEADING_TEXT_SIZE))
                    .on_press(Message::Add)
                    .padding(15);

                let text_row = row![
                    horizontal_space()
                    .width(10),

                    text("Adresse")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

                    horizontal_space(),

                    text("LM-Name")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

                    horizontal_space(),

                    text("Bezeichnung")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

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

            Page::Show => {
                let mut lrm = self.lok_resource_manager.clone();
                let lok = task::block_on(lrm.get_lok(self.state.selected_lok_id.unwrap())).expect("lok not found"); // TODO async show

                let header = header(format!("{}", lok.name));

                let left_column = iced::widget::column!(
                    text("Adresse")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

                    text!("{}", lok.get_address_pretty())
                    .size(frontend::HEADING_TEXT_SIZE),

                    vertical_space(),

                    text("Bezeichnung")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

                    text!("{}", lok.name)
                    .size(frontend::HEADING_TEXT_SIZE),

                    vertical_space(),

                    text("Hersteller")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

                    text!("{}", lok.get_producer_pretty())
                    .size(frontend::HEADING_TEXT_SIZE),
                ).spacing(10);

                let right_column = iced::widget::column![
                    text("LOKmaus-Anzeigename")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

                    text!("{}", lok.get_lokmaus_name_pretty())
                    .size(frontend::HEADING_TEXT_SIZE),

                    vertical_space(),
                    vertical_space(),
                    vertical_space(),
                    vertical_space(),

                    text("Bahnverwaltung")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .font(frontend::font::bold_font()),

                    text!("{}", lok.get_management_pretty())
                    .size(frontend::HEADING_TEXT_SIZE)
                ].spacing(10);


                let cancel_button = button(text("Abbrechen").size(frontend::HEADING_TEXT_SIZE))
                    .on_press(Message::Cancel)
                    .padding(15);

                let edit_button = button(row![frontend::font::edit_icon(), text("Bearbeiten").size(frontend::HEADING_TEXT_SIZE)].align_y(Center))
                    .on_press_with(move || {
                        Message::Edit(self.state.selected_lok_id.clone().unwrap())
                    })
                    .padding(15);

                let remove_button = button(row![frontend::font::delete_icon(), text("Löschen").size(frontend::HEADING_TEXT_SIZE)].align_y(Center))
                    .on_press_with(move || {
                        Message::Remove(self.state.selected_lok_id.clone().unwrap())
                    })
                    .style(button::danger)
                    .padding(15);

                iced::widget::column![
                    header,
                    row![
                        horizontal_space(),
                        left_column,
                        horizontal_space(),
                        right_column,
                        horizontal_space(),
                    ].width(Fill).height(Fill),
                    center(row![
                        horizontal_space(),
                        edit_button,
                        horizontal_space(),
                        remove_button,
                        horizontal_space(),
                        cancel_button,
                        horizontal_space(),
                    ])].width(Fill).into()
            }

            Page::Settings => { text("settings").into() }
        }
    }

    /// Layouts the input mask for adding and editing a Lok.
    /// The message on finish is emitted when the save button was pressed.
    fn lok_data_input_mask(&self, header_text: String, message_on_finish: Message) -> Element<Message> {
        let header = header(header_text);

        let upper_row = row![
            button(image("res/images/blue.png").width(400).content_fit(ContentFit::Cover)).style(button::text),
            column![
                column!(
                text("Bezeichnung")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(frontend::font::bold_font()),

                text_input("Bezeichnung", self.state.name_input.as_str())
                    .id("new-lok-name")
                    .on_input(Message::NameInputChanged)
                    .padding(15)
                    .size(frontend::HEADING_TEXT_SIZE)
                    .align_x(Left),
                ),
                vertical_space(),

                column!(
                text("Analog/Digital")
                    .size(frontend::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(frontend::font::bold_font()),

                checkbox("Analog", !self.state.has_decoder_input)
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(frontend::HEADING_TEXT_SIZE),

                checkbox("Digital", self.state.has_decoder_input)
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(frontend::HEADING_TEXT_SIZE),
            ),
                vertical_space(),
            ]
        ].spacing(20).padding(20);

        let center_row = row![
            column!(
                text("Adresse")
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(frontend::font::bold_font()),

                text_input("Adresse", self.state.address_input.as_str())
                .id("new-lok-address")
                .on_input_maybe(
                    if self.state.has_decoder_input {
                        Some(Message::AddressInputChanged)
                    }
                    else {
                        None
                    }
                 )
                .padding(15)
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left),
            ),
            column![
                text("LOKmaus-Anzeigename")
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(frontend::font::bold_font()),

                text_input("LOKmaus-Name", self.state.lok_maus_name_input.as_str())
                .id("new-lok-short-name")
                .on_input_maybe(
                    if self.state.has_decoder_input {
                        Some(Message::LokMausNameInputChanged)
                    }
                    else {
                        None
                    }
                 )
                .padding(15)
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left),
            ]
        ].spacing(20).padding(20);

        let lower_row = row![
            column![
                text("Hersteller")
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(frontend::font::bold_font()),

                text_input("Hersteller", self.state.producer_input.as_str())
                .id("new-lok-producer")
                .on_input(Message::ProducerInputChanged)
                .padding(15)
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left),
            ],
            column!(
                text("Bahnverwaltung")
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(frontend::font::bold_font()),

                text_input("Bahnverwaltung", self.state.management_input.as_str())
                .id("new-lok-management")
                .on_input(Message::ManagementInputChanged)
                .padding(15)
                .size(frontend::HEADING_TEXT_SIZE)
                .align_x(Left),
            )
        ].spacing(20).padding(20);

        let add_button = button(text("Speichern").size(frontend::HEADING_TEXT_SIZE))
            .on_press(message_on_finish)
            .padding(15);

        let cancel_button = button(text("Abbrechen").size(frontend::HEADING_TEXT_SIZE))
            .on_press(Message::Cancel)
            .padding(15);

        iced::widget::column![
            header,
            column![
                column![
                    upper_row,
                ],
                column![
                    vertical_space(),
                    center_row,
                    vertical_space(),
                    lower_row,
                    vertical_space()
                ]
            ].height(Fill),
            row![
                horizontal_space(),
                add_button,
                horizontal_space(),
                cancel_button,
                horizontal_space(),
            ],
        ].width(Fill).into()
    }
}