use iced::widget::{text, Text};
use iced::{font, Center, Font};

const ICONS: Font = Font::with_name("Iced-Todos-Icons");

pub fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .align_x(Center)
}

pub fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

pub fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}

pub fn bold_font() -> Font {
    Font { weight: font::Weight::Bold, ..Font::default() }
}