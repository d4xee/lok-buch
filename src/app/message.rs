use crate::app::backend::sqlite_backend::SQLiteBackend;
use crate::app::persistent_data::PersistentData;
use iced::Event;
use rfd::MessageDialogResult;

#[derive(Clone, Debug)]
pub enum Message {
    Loaded(PersistentData<SQLiteBackend>),
    Remove(u32),
    NameInputChanged(String),
    AddressInputChanged(i32),
    LokMausNameInputChanged(String),
    ProducerInputChanged(String),
    ManagementInputChanged(String),
    HasDecoderInputChanged(bool),
    SearchInputChanged(String),
    Add,
    AddNewLok,
    Saved(u32),
    Cancel,
    ShowLok(u32),
    Edit(u32),
    InputFailure(MessageDialogResult),
    EditLok,
    Settings,
    EventOccurred(Event),
}