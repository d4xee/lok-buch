use crate::app::message::Message;
use crate::backend::database::preview_lok::PreviewLok;
use crate::frontend::{font, VIEW_NAME_TEXT_SIZE, VIEW_TITLE_TEXT_SIZE};
use iced::widget::{button, container, horizontal_space, row, text, vertical_space, Container};
use iced::{Center, Element, Fill};

pub fn header<'a>(name: String) -> Element<'a, Message> {
    iced::widget::column!(
        text("LOKBUCH")
            .width(Fill)
            .size(VIEW_TITLE_TEXT_SIZE)
            .color([0.5, 0.5, 0.5])
            .align_x(Center)
            .font(font::bold_font()),

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

/// Returns an inputted PreviewLok as a custom widget.
/// This is used for the main page.
pub fn preview_widget<'a>(preview_data: PreviewLok) -> Container<'a, Message> {
    let preview_id = preview_data.get_id();

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
            text!("{}", preview_data.get_address_pretty()),
            horizontal_space(),
        ],

        row![
            text!("{}", preview_data.get_lokmaus_name_pretty()),
            horizontal_space(),
        ],

        row![
            text!("{}", preview_data.get_name_pretty()),
            horizontal_space(),
        ],

        horizontal_space(),
        button_row
        ])
        .padding(10)
        .style(container::rounded_box)
        .width(Fill)
}
