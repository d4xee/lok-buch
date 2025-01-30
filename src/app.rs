mod state;
mod stored_data;
pub(crate) mod message;
mod page;
pub mod backend;
pub mod ui;

use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::state::State;
use crate::app::stored_data::StoredData;
use backend::resource_manager::LokResourceManager;
use backend::sqlite_backend::SQLiteBackend;
use iced::{Element, Task};
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

    pub fn change_page_to(&mut self, page: Pages) {
        self.page = page;
    }
}