mod state;
mod stored_data;
pub(crate) mod message;
mod page;

use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::state::State;
use crate::app::stored_data::StoredData;
use crate::backend::resource_manager::LokResourceManager;
use crate::backend::sqlite_backend::SQLiteBackend;
use crate::ui;
use crate::ui::widgets::header;
use iced::widget::{button, checkbox, column, horizontal_space, image, row, text, text_input, vertical_space};
use iced::{ContentFit, Element, Fill, Left, Task};
use std::any::Any;
use std::ops::{Deref, DerefMut};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DB_URL: &str = "sqlite://data/lokbuch.db";

pub struct Lokbuch {
    state: State,
    lok_resource_manager: LokResourceManager<SQLiteBackend>,
    moving_icon_frames: iced_gif::Frames,
    page: Pages,
}

impl Lokbuch {
    pub(crate) fn new() -> (Self, Task<Message>) {
        (Lokbuch {
            page: Pages::Loading,
            state: State::default(),
            lok_resource_manager: LokResourceManager::default(),
            moving_icon_frames: ui::moving_icon_frames(),
        },
         Task::perform(StoredData::init_backend(DB_URL), Message::Loaded))
    }

    pub(crate) fn title(&self) -> String {
        format!("Lokbuch v{}", VERSION)
    }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        self.page.as_page_struct().update(self, message)
    }

    pub(crate) fn view(&self) -> Element<'_, Message> {
        self.page.as_page_struct().view(self)
    }

    /// Layouts the input mask for adding and editing a Lok.
    /// The message on finish is emitted when the save button was pressed.
    pub fn lok_data_input_mask(&self, header_text: String, message_on_finish: Message) -> Element<Message> {
        let header = header(header_text);

        let upper_row = row![
            button(image("res/images/blue.png").width(400).content_fit(ContentFit::Cover)).style(button::text),
            column![
                column!(
                text("Bezeichnung")
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(ui::font::bold_font()),

                text_input("Bezeichnung", self.state.name_input.as_str())
                    .id("new-lok-name")
                    .on_input(Message::NameInputChanged)
                    .padding(15)
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left),
                ),
                vertical_space(),

                column!(
                text("Analog/Digital")
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(ui::font::bold_font()),

                checkbox("Analog", !self.state.has_decoder_input)
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(ui::HEADING_TEXT_SIZE),

                checkbox("Digital", self.state.has_decoder_input)
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(ui::HEADING_TEXT_SIZE),
            ),
                vertical_space(),
            ]
        ].spacing(20).padding(20);

        let center_row = row![
            column!(
                text("Adresse")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(ui::font::bold_font()),

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
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            ),
            column![
                text("LOKmaus-Anzeigename")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(ui::font::bold_font()),

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
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            ]
        ].spacing(20).padding(20);

        let lower_row = row![
            column![
                text("Hersteller")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(ui::font::bold_font()),

                text_input("Hersteller", self.state.producer_input.as_str())
                .id("new-lok-producer")
                .on_input(Message::ProducerInputChanged)
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            ],
            column!(
                text("Bahnverwaltung")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(ui::font::bold_font()),

                text_input("Bahnverwaltung", self.state.management_input.as_str())
                .id("new-lok-management")
                .on_input(Message::ManagementInputChanged)
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            )
        ].spacing(20).padding(20);

        let add_button = button(text("Speichern").size(ui::HEADING_TEXT_SIZE))
            .on_press(message_on_finish)
            .padding(15);

        let cancel_button = button(text("Abbrechen").size(ui::HEADING_TEXT_SIZE))
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

    pub fn change_page_to(&mut self, page: Pages) {
        self.page = page;
    }
}