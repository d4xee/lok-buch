use iced::{Center, Element, Fill};
use iced::widget::{button, column, container, horizontal_space, row, text, vertical_space, Container};
use crate::app::Message;
use crate::database::lok::Lok;

const VIEW_NAME_TEXT_SIZE: u16 = 25;
const VIEW_TITLE_TEXT_SIZE: u16 = 75;
pub const TEXT_SIZE: u16 = 20;

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

pub fn lok_as_element(lok: &Lok) -> Container<Message> {
    let lok_edit = lok.clone();
    let lok_remove = lok.clone();

    let button_row = row![
        horizontal_space(),
        button(font::edit_icon())
        .on_press_with(move || {
            Message::Edit(lok_edit.clone())
        })
        .style(button::secondary),
        button(font::delete_icon())
        .on_press_with(move || {
            Message::Remove(lok_remove.clone())
        })
        .style(button::danger)
    ]
        .spacing(10)
        .width(100);

    container(row![
        row![
            text!("{}", lok.address),
            horizontal_space(),
        ],

        row![
            text!("{}", lok.lokmaus_name),
            horizontal_space(),
        ],

        row![
            text!("{}", lok.name),
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
    use iced::{Center, Font};
    use iced::widget::{text, Text};

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