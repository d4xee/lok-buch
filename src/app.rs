use crate::backend::resource_manager::LokResourceManager;
use crate::backend::sqlite_backend::SQLiteBackend;
use crate::database::lok::Lok;
use crate::{init_backend, ui};
use async_std::task;
use iced::widget::{button, center, column, container, horizontal_space, keyed_column, row, scrollable, text, text_input, vertical_space};
use iced::{Center, Element, Fill, Left, Task};
use rfd::MessageDialogResult;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Lokbuch {
    view: View,
    state: State,
    lok_resource_manager: LokResourceManager<SQLiteBackend>,
}

pub enum View {
    Loading,
    Home,
    Add,
    Edit,
    Show,
    Settings,
}

#[derive(Clone, Debug)]
pub struct SavedData {
    pub(crate) lrm: LokResourceManager<SQLiteBackend>,
}

#[derive(Clone)]
pub struct State {
    name_input: String,
    address_input: String,
    lok_maus_name_input: String,
    producer_input: String,
    management_input: String,
    search_input: String,
    selected_lok_id: Option<u32>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Loaded(SavedData),
    Remove(u32),
    NameInputChanged(String),
    AddressInputChanged(String),
    LokMausNameInputChanged(String),
    ProducerInputChanged(String),
    ManagementInputChanged(String),
    SearchInputChanged(String),
    Add,
    CreateLok,
    Saved(u32),
    Search,
    Cancel,
    ShowLok(u32),
    Edit(u32),
    InputFailure(MessageDialogResult),
    Edited,
}

impl State {
    /// Clears all internal state variables.
    /// These are usually the user inputs.
    fn clear(&mut self) {
        self.name_input.clear();
        self.address_input.clear();
        self.lok_maus_name_input.clear();
        self.producer_input.clear();
        self.management_input.clear();
        self.search_input.clear();
        self.selected_lok_id = None;
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            name_input: String::default(),
            address_input: String::default(),
            lok_maus_name_input: String::default(),
            producer_input: String::default(),
            management_input: String::default(),
            search_input: String::default(),
            selected_lok_id: None,
        }
    }
}

impl Lokbuch {
    pub(crate) fn new() -> (Self, Task<Message>) {
        (Lokbuch {
            view: View::Loading,
            state: State::default(),
            lok_resource_manager: LokResourceManager::default(),
        },
         Task::perform(init_backend(), Message::Loaded))
    }

    pub(crate) fn title(&self) -> String {
        format!("Lokbuch v{}", VERSION)
    }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        match self.view {
            View::Loading => {
                match message {
                    Message::Loaded(saved_data) => {
                        self.lok_resource_manager = saved_data.lrm.clone();
                        self.view = View::Home;
                    }
                    _ => {}
                }
                Task::none()
            }

            View::Home | View::Show => {
                match message {
                    Message::Add => {
                        self.view = View::Add;
                    }

                    Message::SearchInputChanged(search_input) => {
                        self.state.search_input = search_input;
                    }

                    Message::Search => {}

                    Message::ShowLok(id) => {
                        self.state = State {
                            selected_lok_id: Some(id),
                            ..State::default()
                        };

                        self.view = View::Show;
                    }

                    Message::Cancel => {
                        self.view = View::Home;
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

                        self.view = View::Edit;
                    }

                    Message::Remove(id) => {
                        task::block_on(self.lok_resource_manager.remove_lok(id)); // TODO async remove

                        self.view = View::Home;
                    }

                    _ => {}
                }
                Task::none()
            }

            View::Add => {
                match message {
                    Message::NameInputChanged(name) => {
                        self.state.name_input = name;
                    }
                    Message::AddressInputChanged(address) => {
                        self.state.address_input = address;
                    }
                    Message::LokMausNameInputChanged(name) => {
                        self.state.lok_maus_name_input = name;
                    }
                    Message::ProducerInputChanged(producer) => {
                        self.state.producer_input = producer;
                    }
                    Message::ManagementInputChanged(management) => {
                        self.state.management_input = management;
                    }
                    Message::CreateLok => {
                        println!("Create Lok: Speichern pressed");

                        if self.state.name_input.is_empty() || self.state.address_input.is_empty() || self.state.lok_maus_name_input.is_empty() {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse, Bezeichnung und LOKmaus-Anzeigename dürfen nicht leer sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let is_num = self.state.address_input.clone().parse::<i32>().is_ok();

                        if !is_num {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse muss eine Zahl sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        if self.state.lok_maus_name_input.len() > 5 {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Der LOKmaus-Anzeigename darf nicht länger als 5 Zeichen sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let new_lok = Lok::new_from_raw_data(
                            self.state.name_input.clone(),
                            self.state.address_input.clone().parse::<i32>().unwrap(),
                            self.state.lok_maus_name_input.clone().to_string().to_ascii_uppercase(),
                            self.state.producer_input.clone(),
                            self.state.management_input.clone(),
                        );

                        self.state.clear();

                        task::block_on(self.lok_resource_manager.add_lok(new_lok.clone()));

                        self.view = View::Home;
                    }
                    Message::Cancel => {
                        self.view = View::Home;
                    }

                    Message::Saved(id) => {
                        self.view = View::Home;
                    }
                    _ => {}
                }
                Task::none()
            }

            View::Edit => {
                match message {
                    Message::NameInputChanged(name) => {
                        self.state.name_input = name;
                    }
                    Message::AddressInputChanged(address) => {
                        self.state.address_input = address;
                    }
                    Message::LokMausNameInputChanged(name) => {
                        self.state.lok_maus_name_input = name;
                    }
                    Message::ProducerInputChanged(producer) => {
                        self.state.producer_input = producer;
                    }
                    Message::ManagementInputChanged(management) => {
                        self.state.management_input = management;
                    }
                    Message::Edited => {
                        if self.state.name_input.is_empty() || self.state.address_input.is_empty() || self.state.lok_maus_name_input.is_empty() {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse, Bezeichnung und LOKmaus-Anzeigename dürfen nicht leer sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let is_num = self.state.address_input.clone().parse::<i32>().is_ok();

                        if !is_num {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse muss eine Zahl sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        if self.state.lok_maus_name_input.len() > 5 {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Der LOKmaus-Anzeigename darf nicht länger als 5 Zeichen sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let new_lok = Lok::new_from_raw_data(
                            self.state.name_input.clone(),
                            self.state.address_input.clone().parse::<i32>().unwrap(),
                            self.state.lok_maus_name_input.clone().to_string().to_ascii_uppercase(),
                            self.state.producer_input.clone(),
                            self.state.management_input.clone(),
                        );

                        let old_lok_id = self.state.selected_lok_id.clone().unwrap();

                        task::block_on(self.lok_resource_manager.update_lok(old_lok_id, new_lok));

                        self.state.clear();
                        self.view = View::Home;
                    }
                    Message::Cancel => {
                        self.state.clear();
                        self.view = View::Home;
                    }
                    _ => {}
                }
                Task::none()
            }

            View::Settings => { Task::none() }
        }
    }

    pub(crate) fn view(&self) -> Element<'_, Message> {
        match self.view {
            View::Loading => {
                center(text("Loading...").width(Fill).align_x(Center).size(50)).into()
            }

            View::Add => {
                let header = ui::view_header(String::from("Hinzufügen"));

                let upper_row = row![
                    column!(
                        text("Adresse")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Adresse", self.state.address_input.as_str())
                    .id("new-lok-address")
                    .on_input(Message::AddressInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ),
                    column![
                        text("LOKmaus-Anzeigename")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("LOKmaus-Name", self.state.lok_maus_name_input.as_str())
                    .id("new-lok-short-name")
                    .on_input(Message::LokMausNameInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ]
                ].spacing(10);

                let center_row = row![
                    column!(
                        text("Bezeichnung")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Bezeichnung", self.state.name_input.as_str())
                    .id("new-lok-name")
                    .on_input(Message::NameInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ),
                    column![
                        horizontal_space(),
                        horizontal_space()
                    ]
                ].spacing(10);

                let lower_row = row![
                    column![
                        text("Hersteller")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Hersteller", self.state.producer_input.as_str())
                    .id("new-lok-producer")
                    .on_input(Message::ProducerInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ],
                    column!(
                        text("Bahnverwaltung")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Bahnverwaltung", self.state.management_input.as_str())
                    .id("new-lok-management")
                    .on_input(Message::ManagementInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    )
                ].spacing(10);

                let add_button = button(text("Speichern").size(ui::TEXT_SIZE))
                    .on_press(Message::CreateLok)
                    .padding(15);

                let cancel_button = button(text("Abbrechen").size(ui::TEXT_SIZE))
                    .on_press(Message::Cancel)
                    .padding(15);

                iced::widget::column![header, column![upper_row, vertical_space(), center_row, vertical_space(), lower_row], center(row![horizontal_space(),add_button, horizontal_space(), cancel_button, horizontal_space(),])].width(Fill).into()
            }

            View::Edit => {
                let header = ui::view_header(String::from("Bearbeiten"));

                let upper_row = row![
                    column!(
                        text("Adresse")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Adresse", self.state.address_input.as_str())
                    .id("new-lok-address")
                    .on_input(Message::AddressInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ),
                    column![
                        text("LOKmaus-Anzeigename")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("LOKmaus-Name", self.state.lok_maus_name_input.as_str())
                    .id("new-lok-short-name")
                    .on_input(Message::LokMausNameInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ]
                ].spacing(10);

                let center_row = row![
                    column!(
                        text("Bezeichnung")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Bezeichnung", self.state.name_input.as_str())
                    .id("new-lok-name")
                    .on_input(Message::NameInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ),
                    column![
                        horizontal_space(),
                        horizontal_space()
                    ]
                ].spacing(10);

                let lower_row = row![
                    column![
                        text("Hersteller")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Hersteller", self.state.producer_input.as_str())
                    .id("new-lok-producer")
                    .on_input(Message::ProducerInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    ],
                    column!(
                        text("Bahnverwaltung")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Bahnverwaltung", self.state.management_input.as_str())
                    .id("new-lok-management")
                    .on_input(Message::ManagementInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),
                    )
                ].spacing(10);

                let add_button = button(text("Speichern").size(ui::TEXT_SIZE))
                    .on_press(Message::Edited)
                    .padding(15);

                let cancel_button = button(text("Abbrechen").size(ui::TEXT_SIZE))
                    .on_press(Message::Cancel)
                    .padding(15);

                iced::widget::column![
                    header,
                    column![
                        upper_row,
                        vertical_space(),
                        center_row,
                        vertical_space(),
                        lower_row
                    ],
                    center(
                        row![
                            horizontal_space(),
                            add_button,
                            horizontal_space(),
                            cancel_button,
                            horizontal_space(),
                        ]
                    )].width(Fill).into()
            }

            View::Home => {
                let num_of_loks = self.lok_resource_manager.number_of_loks();
                let mut previews = self.lok_resource_manager.get_all_previews().clone();

                let header = ui::view_header(format!("{} Loks vorhanden", num_of_loks));

                let input_search = text_input("Suchen...", self.state.search_input.as_str())
                    .id("lok-search")
                    .on_input(Message::SearchInputChanged)
                    .padding(15)
                    .size(ui::TEXT_SIZE)
                    .align_x(Center);


                let search_button = button(text("Suchen").size(ui::TEXT_SIZE))
                    .on_press(Message::Search)
                    .padding(15);

                let add_button = button(text("Neue Lok hinzufügen").size(ui::TEXT_SIZE))
                    .on_press(Message::Add)
                    .padding(15);

                let text_row = row![
                    horizontal_space()
                    .width(10),

                    text("Adresse")
                    .size(ui::TEXT_SIZE),

                    horizontal_space(),

                    text("LM-Name")
                    .size(ui::TEXT_SIZE),

                    horizontal_space(),

                    text("Bezeichnung")
                    .size(ui::TEXT_SIZE),

                    horizontal_space(),

                    horizontal_space(),

                    horizontal_space()
                ];

                let loks = keyed_column(
                    previews.into_iter().map(move |item| {
                        let preview = item.clone();
                        (0, iced::widget::column!(
                        button(ui::preview_as_ui_element(preview.clone()))
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

            View::Show => {
                let mut lrm = self.lok_resource_manager.clone();
                let lok = task::block_on(lrm.get_lok(self.state.selected_lok_id.unwrap())).expect("lok not found"); // TODO async show

                let header = ui::view_header(format!("{}", lok.name));

                let left_column = iced::widget::column!(
                    text("Adresse")
                    .size(ui::TEXT_SIZE),

                    text!("{}", lok.get_address_pretty())
                    .size(ui::TEXT_SIZE),

                    vertical_space(),

                    text("Bezeichnung")
                    .size(ui::TEXT_SIZE),

                    text!("{}", lok.name)
                    .size(ui::TEXT_SIZE),

                    vertical_space(),

                    text("Hersteller")
                    .size(ui::TEXT_SIZE),

                    text!("{}", lok.get_producer_pretty())
                    .size(ui::TEXT_SIZE),
                ).spacing(10);

                let right_column = iced::widget::column![
                    text("LOKmaus-Anzeigename")
                    .size(ui::TEXT_SIZE),

                    text!("{}", lok.get_lokmaus_name_pretty())
                    .size(ui::TEXT_SIZE),

                    vertical_space(),
                    vertical_space(),
                    vertical_space(),
                    vertical_space(),

                    text("Bahnverwaltung")
                    .size(ui::TEXT_SIZE),

                    text!("{}", lok.get_management_pretty())
                    .size(ui::TEXT_SIZE)
                ].spacing(10);


                let cancel_button = button(text("Abbrechen").size(ui::TEXT_SIZE))
                    .on_press(Message::Cancel)
                    .padding(15);

                let edit_button = button(row![ui::font::edit_icon(), text("Bearbeiten").size(ui::TEXT_SIZE)].align_y(Center))
                    .on_press_with(move || {
                        Message::Edit(self.state.selected_lok_id.clone().unwrap())
                    })
                    .padding(15);

                let remove_button = button(row![ui::font::delete_icon(), text("Löschen").size(ui::TEXT_SIZE)].align_y(Center))
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

            View::Settings => { text("settings").into() }
        }
    }
}