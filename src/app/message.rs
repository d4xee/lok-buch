use crate::app::stored_data::StoredData;
use iced::window::Id;
use rfd::MessageDialogResult;

#[derive(Clone, Debug)]
pub enum Message {
    Loaded(StoredData),
    Remove(u32),
    NameInputChanged(String),
    AddressInputChanged(String),
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
    GotId(Option<Id>),
}