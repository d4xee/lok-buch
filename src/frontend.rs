pub mod widgets;
pub mod font;

const VIEW_NAME_TEXT_SIZE: u16 = 25;
const VIEW_TITLE_TEXT_SIZE: u16 = 75;
pub const HEADING_TEXT_SIZE: u16 = 20;

pub const NO_DATA_AVAILABLE_TEXT: &str = "---";

pub const ICON_PATH: &str = "res/images/icon.png";

pub fn moving_icon_frames() -> iced_gif::Frames {
    iced_gif::Frames::from_bytes(include_bytes!("../res/images/icon_move.gif").into()).expect("Decoding gif failed!")
}