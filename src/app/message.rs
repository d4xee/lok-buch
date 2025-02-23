use crate::app::backend::sqlite_backend::SQLiteBackend;
use crate::app::persistent_data::PersistentData;
use iced::Event;
use rfd::MessageDialogResult;

#[derive(Clone, Debug)]
pub enum Message {
    Add,
    AddNewLok,
    AddressInputChanged(i32),
    Cancel,
    Edit(u32),
    EditLok,
    EventOccurred(Event),
    HasDecoderInputChanged(bool),
    InputFailure(MessageDialogResult),
    LanguageSelected(usize, &'static str),
    Loaded(PersistentData<SQLiteBackend>),
    LokMausNameInputChanged(String),
    ManagementInputChanged(String),
    NameInputChanged(String),
    ProducerInputChanged(String),
    Remove(u32),
    Saved(u32),
    SearchInputChanged(String),
    Settings,
    ShowLok(u32),
}