pub mod widgets;
pub mod font;

const VIEW_NAME_TEXT_SIZE: u32 = 25;
const VIEW_TITLE_TEXT_SIZE: u32 = 75;
pub const HEADING_TEXT_SIZE: f32 = 20.0;

pub const SVG_ICON_HEIGHT: u32 = 25;

pub const NO_DATA_AVAILABLE_TEXT: &str = "---";

pub const ICON_PATH: &str = "res/images/icon.png";

pub const DEFAULT_LOCO_IMAGE_PATH: &str = "res/images/default.png";

pub fn moving_icon_frames() -> iced_gif::Frames {
    iced_gif::Frames::from_bytes(include_bytes!("../../res/images/icon_move.gif").into()).expect("Decoding gif failed!")
}

pub enum SvgIcon {
    Back,
    Edit,
    Gear,
    Plus,
    Trash,
}

impl SvgIcon {
    pub(crate) fn get_file_path(&self) -> &'static str {
        match self {
            SvgIcon::Back => { "res/images/svg/back.svg" }
            SvgIcon::Edit => { "res/images/svg/edit.svg" }
            SvgIcon::Gear => { "res/images/svg/gear.svg" }
            SvgIcon::Plus => { "res/images/svg/plus.svg" }
            SvgIcon::Trash => { "res/images/svg/trash.svg" }
        }
    }
}