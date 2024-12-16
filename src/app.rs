use crate::backend::resource_manager::LokResourceManager;
use crate::backend::sqlite_backend::SQLiteBackend;
use crate::database::lok::Lok;
use crate::{init_backend, ui};
use async_std::task;
use iced::widget::{button, center, column, container, horizontal_space, keyed_column, row, scrollable, text, text_input, vertical_space};
use iced::{Center, Element, Fill, Left, Task};
use rfd::MessageDialogResult;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum Lokbuch {
    LoadingView,
    HomeView(State),
    AddView(State),
    EditView(State),
    ShowView(State),
    SettingsView(State),
}

#[derive(Clone, Debug)]
pub struct SavedData {
    pub(crate) lrm: LokResourceManager<SQLiteBackend>,
}

#[derive(Clone)]
pub struct State {
    lrm: LokResourceManager<SQLiteBackend>,
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

impl Default for State {
    fn default() -> Self {
        State {
            lrm: LokResourceManager::default(),
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
        (Lokbuch::LoadingView,
         Task::perform(init_backend(), Message::Loaded))
    }

    pub(crate) fn title(&self) -> String {
        format!("Lokbuch v{}", VERSION)
    }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            Lokbuch::LoadingView => {
                match message {
                    Message::Loaded(saved_data) => {
                        *self = Lokbuch::HomeView(
                            State {
                                lrm: saved_data.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }
                    _ => {}
                }
                Task::none()
            }

            Lokbuch::HomeView(state) | Lokbuch::ShowView(state) => {
                match message {
                    Message::Add => {
                        *self = Lokbuch::AddView(
                            State {
                                lrm: state.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }

                    Message::SearchInputChanged(search_input) => {
                        state.search_input = search_input;
                    }

                    Message::Search => {}

                    Message::ShowLok(id) => {
                        *self = Lokbuch::ShowView(
                            State {
                                lrm: state.lrm.clone(),
                                selected_lok_id: Some(id),
                                ..State::default()
                            }
                        );
                    }

                    Message::Cancel => {
                        *self = Lokbuch::HomeView(
                            State {
                                lrm: state.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }

                    Message::Edit(id) => {
                        let lok = task::block_on(state.lrm.get_lok(id)).expect("Lok does not exist!"); // TODO async edit

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

                        *self = Lokbuch::EditView(
                            State {
                                lrm: state.lrm.clone(),
                                selected_lok_id: Some(id),
                                name_input,
                                address_input,
                                lok_maus_name_input,
                                producer_input,
                                management_input,
                                ..State::default()
                            }
                        );
                    }

                    Message::Remove(id) => {
                        task::block_on(state.lrm.remove_lok(id)); // TODO async remove

                        *self = Lokbuch::HomeView(
                            State {
                                lrm: state.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }

                    _ => {}
                }
                Task::none()
            }

            Lokbuch::AddView(state) => {
                match message {
                    Message::NameInputChanged(name) => {
                        state.name_input = name;
                    }
                    Message::AddressInputChanged(address) => {
                        state.address_input = address;
                    }
                    Message::LokMausNameInputChanged(name) => {
                        state.lok_maus_name_input = name;
                    }
                    Message::ProducerInputChanged(producer) => {
                        state.producer_input = producer;
                    }
                    Message::ManagementInputChanged(management) => {
                        state.management_input = management;
                    }
                    Message::CreateLok => {
                        println!("Create Lok: Speichern pressed");

                        if state.name_input.is_empty() || state.address_input.is_empty() || state.lok_maus_name_input.is_empty() {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse, Bezeichnung und LOKmaus-Anzeigename dürfen nicht leer sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let is_num = state.address_input.clone().parse::<i32>().is_ok();

                        if !is_num {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse muss eine Zahl sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        if state.lok_maus_name_input.len() > 5 {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Der LOKmaus-Anzeigename darf nicht länger als 5 Zeichen sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let new_lok = Lok::new_from_raw_data(
                            state.name_input.clone(),
                            state.address_input.clone().parse::<i32>().unwrap(),
                            state.lok_maus_name_input.clone().to_string().to_ascii_uppercase(),
                            state.producer_input.clone(),
                            state.management_input.clone(),
                        );

                        state.name_input.clear();
                        state.address_input.clear();
                        state.lok_maus_name_input.clear();
                        state.producer_input.clear();
                        state.management_input.clear();

                        return Task::perform(state.lrm.add_lok(new_lok.clone()), Message::Saved);
                    }
                    Message::Cancel => {
                        *self = Lokbuch::HomeView(
                            State {
                                lrm: state.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }

                    Message::Saved(id) => {
                        *self = Lokbuch::HomeView(
                            State {
                                lrm: state.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }
                    _ => {}
                }
                Task::none()
            }

            Lokbuch::EditView(state) => {
                match message {
                    Message::NameInputChanged(name) => {
                        state.name_input = name;
                    }
                    Message::AddressInputChanged(address) => {
                        state.address_input = address;
                    }
                    Message::LokMausNameInputChanged(name) => {
                        state.lok_maus_name_input = name;
                    }
                    Message::ProducerInputChanged(producer) => {
                        state.producer_input = producer;
                    }
                    Message::ManagementInputChanged(management) => {
                        state.management_input = management;
                    }
                    Message::Edited => {
                        if state.name_input.is_empty() || state.address_input.is_empty() || state.lok_maus_name_input.is_empty() {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse, Bezeichnung und LOKmaus-Anzeigename dürfen nicht leer sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let is_num = state.address_input.clone().parse::<i32>().is_ok();

                        if !is_num {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Adresse muss eine Zahl sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        if state.lok_maus_name_input.len() > 5 {
                            let res = rfd::AsyncMessageDialog::new()
                                .set_title("Eingabefehler")
                                .set_description("Der LOKmaus-Anzeigename darf nicht länger als 5 Zeichen sein!")
                                .set_buttons(rfd::MessageButtons::Ok);

                            return Task::perform(res.show(), Message::InputFailure);
                        }

                        let new_lok = Lok::new_from_raw_data(
                            state.name_input.clone(),
                            state.address_input.clone().parse::<i32>().unwrap(),
                            state.lok_maus_name_input.clone().to_string().to_ascii_uppercase(),
                            state.producer_input.clone(),
                            state.management_input.clone(),
                        );

                        let old_lok_id = state.selected_lok_id.clone().unwrap();

                        task::block_on(state.lrm.update_lok(old_lok_id, new_lok));

                        *self = Lokbuch::HomeView(
                            State {
                                lrm: state.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }
                    Message::Cancel => {
                        *self = Lokbuch::HomeView(
                            State {
                                lrm: state.lrm.clone(),
                                ..State::default()
                            }
                        );
                    }
                    _ => {}
                }
                Task::none()
            }

            Lokbuch::SettingsView(_) => { Task::none() }
        }
    }

    pub(crate) fn view(&self) -> Element<'_, Message> {
        match self {
            Lokbuch::LoadingView => {
                center(text("Loading...").width(Fill).align_x(Center).size(50)).into()
            }

            Lokbuch::AddView(State {
                                 name_input,
                                 address_input,
                                 lok_maus_name_input,
                                 producer_input,
                                 management_input,
                                 ..
                             }) => {
                let header = ui::view_header(String::from("Hinzufügen"));

                let upper_row = row![
                    column!(
                        text("Adresse")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Adresse", address_input)
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

                    text_input("LOKmaus-Name", lok_maus_name_input)
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

                    text_input("Bezeichnung", name_input)
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

                    text_input("Hersteller", producer_input)
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

                    text_input("Bahnverwaltung", management_input)
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

            Lokbuch::EditView(State {
                                  name_input,
                                  address_input,
                                  lok_maus_name_input,
                                  producer_input,
                                  management_input,
                                  ..
                              }) => {
                let header = ui::view_header(String::from("Bearbeiten"));

                let upper_row = row![
                    column!(
                        text("Adresse")
                    .size(ui::TEXT_SIZE)
                    .align_x(Left),

                    text_input("Adresse", address_input)
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

                    text_input("LOKmaus-Name", lok_maus_name_input)
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

                    text_input("Bezeichnung", name_input)
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

                    text_input("Hersteller", producer_input)
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

                    text_input("Bahnverwaltung", management_input)
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

            Lokbuch::HomeView(State {
                                  lrm,
                                  search_input,
                                  ..
                              }) => {
                let num_of_loks = lrm.number_of_loks();
                let previews = lrm.get_all_previews();

                let header = ui::view_header(format!("{} Loks vorhanden", num_of_loks));

                let input_search = text_input("Suchen...", search_input)
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
                    previews.clone().iter().map(|item| (0, iced::widget::column!(
                        button(ui::preview_as_ui_element(item))
                        .style(button::text)
                        .on_press_with(|| {
                            Message::ShowLok(item.get_id())
                        }),
                        vertical_space()
                        .height(10))
                        .into()))
                ).width(Fill);

                let content = iced::widget::column![header, row![input_search, search_button], add_button, text_row, scrollable(container(loks))].align_x(Center).spacing(20).width(Fill);

                content.into()
            }

            Lokbuch::ShowView(state) => {
                let mut state = state.clone();
                let lok = task::block_on(state.lrm.get_lok(state.selected_lok_id.unwrap())).expect("lok not found"); // TODO async show

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
                        Message::Edit(state.selected_lok_id.clone().unwrap())
                    })
                    .padding(15);

                let remove_button = button(row![ui::font::delete_icon(), text("Löschen").size(ui::TEXT_SIZE)].align_y(Center))
                    .on_press_with(move || {
                        Message::Remove(state.selected_lok_id.clone().unwrap())
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

            Lokbuch::SettingsView(_) => { text("settings").into() }
        }
    }
}