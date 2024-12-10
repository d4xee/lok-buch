use crate::app::Message;
use crate::database::preview_lok::PreviewLok;
use iced::widget::{button, column, container, horizontal_space, row, text, vertical_space, Container};
use iced::{Center, Element, Fill};

const VIEW_NAME_TEXT_SIZE: u16 = 25;
const VIEW_TITLE_TEXT_SIZE: u16 = 75;
pub const TEXT_SIZE: u16 = 20;

pub const NO_DATA_AVAILABLE_TEXT: &str = "---";

pub fn view_header<'a>(name: String) -> Element<'a, Message> {
    column!(
        text("LOKBUCH")
            .width(Fill)
            .size(VIEW_TITLE_TEXT_SIZE)
            .color([0.5, 0.5, 0.5])
            .align_x(Center),

        text(name)
            .width(Fill)
            .size(VIEW_NAME_TEXT_SIZE)
            .color([0.5, 0.5, 0.5])
            .align_x(Center),

        vertical_space()
        .height(15)
    )
        .spacing(20)
        .width(Fill)
        .align_x(Center)
        .into()
}

pub fn preview_as_ui_element(preview: &PreviewLok) -> Container<Message> {
    let preview_id = preview.get_id();

    let button_row = row![
        horizontal_space(),
        button(font::edit_icon())
        .on_press_with(move || {
            Message::Edit(preview_id)
        })
        .style(button::secondary),
        button(font::delete_icon())
        .on_press_with(move || {
            Message::Remove(preview_id)
        })
        .style(button::danger)
    ]
        .spacing(10)
        .width(100);

    container(row![
        row![
            text!("{}", preview.get_address_pretty()),
            horizontal_space(),
        ],

        row![
            text!("{}", preview.get_lokmaus_name_pretty()),
            horizontal_space(),
        ],

        row![
            text!("{}", preview.get_name_pretty()),
            horizontal_space(),
        ],

        horizontal_space(),
        button_row
        ])
        .padding(10)
        .style(container::rounded_box)
        .width(Fill)
}

pub mod font {
    use iced::widget::{text, Text};
    use iced::{Center, Font};

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
}