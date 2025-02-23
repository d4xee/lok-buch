mod state;
mod persistent_data;
pub(crate) mod message;
mod page;
pub mod backend;
pub mod ui;
mod settings;

use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::persistent_data::PersistentData;
use crate::app::settings::Settings;
use crate::app::state::State;
use backend::resource_manager::LokResourceManager;
use backend::sqlite_backend::SQLiteBackend;
use iced::{event, window, Element, Event, Subscription, Task};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DB_URL: &str = "sqlite://data/lokbuch.db";

pub(crate) struct Lokbuch {
    state: State,
    lok_resource_manager: LokResourceManager<SQLiteBackend>,
    moving_icon_frames: iced_gif::Frames,
    page: Pages,
    settings: Settings,
}

impl Lokbuch {
    pub(crate) fn new() -> (Self, Task<Message>) {
        (Lokbuch {
            page: Pages::Loading,
            state: State::default(),
            lok_resource_manager: LokResourceManager::default(),
            moving_icon_frames: ui::moving_icon_frames(),
            settings: Settings::default(),
        },
         Task::batch(vec![
             Task::perform(PersistentData::init_app_and_backend(DB_URL), Message::Loaded),
             iced::window::get_latest().and_then(move |id| iced::window::toggle_maximize(id))
         ]))
    }

    pub(crate) fn title(&self) -> String {
        format!("Lokbuch v{}", VERSION)
    }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    self.settings.save();
                    window::get_latest().and_then(window::close)
                } else {
                    Task::none()
                }
            }

            Message::Settings => {
                self.state = State {
                    ..State::default()
                };

                self.change_page_to(Pages::Settings);
                Task::none()
            }

            _ => self.page.as_page_struct().update(self, message)
        }
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    pub(crate) fn view(&self) -> Element<'_, Message> {
        self.page.as_page_struct().view(self)
    }

    pub(crate) fn change_page_to(&mut self, page: Pages) {
        self.page = page;
    }
}