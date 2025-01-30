use crate::app::backend::database::preview_lok::PreviewLok;
use crate::app::message::Message;
use crate::app::ui::{font, VIEW_NAME_TEXT_SIZE, VIEW_TITLE_TEXT_SIZE};
use crate::app::{ui, Lokbuch};
use iced::widget::{button, checkbox, column, container, horizontal_space, image, row, text, text_input, vertical_space, Container};
use iced::{Center, ContentFit, Element, Fill, Left};

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

/// Layouts the input mask for adding and editing a Lok.
/// The message on finish is emitted when the save button was pressed.
pub fn lok_data_input_mask(lokbuch: &Lokbuch, header_text: String, message_on_finish: Message) -> Element<Message> {
    let header = header(header_text);

    let upper_row = row![
            button(image("res/images/blue.png").width(400).content_fit(ContentFit::Cover)).style(button::text),
            column![
                column!(
                text("Bezeichnung")
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(font::bold_font()),

                text_input("Bezeichnung", lokbuch.state.name_input.as_str())
                    .id("new-lok-name")
                    .on_input(Message::NameInputChanged)
                    .padding(15)
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left),
                ),
                vertical_space(),

                column!(
                text("Analog/Digital")
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(font::bold_font()),

                checkbox("Analog", !lokbuch.state.has_decoder_input)
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(ui::HEADING_TEXT_SIZE),

                checkbox("Digital", lokbuch.state.has_decoder_input)
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(ui::HEADING_TEXT_SIZE),
            ),
                vertical_space(),
            ]
        ].spacing(20).padding(20);

    let center_row = row![
            column!(
                text("Adresse")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(font::bold_font()),

                text_input("Adresse", lokbuch.state.address_input.as_str())
                .id("new-lok-address")
                .on_input_maybe(
                    if lokbuch.state.has_decoder_input {
                        Some(Message::AddressInputChanged)
                    }
                    else {
                        None
                    }
                 )
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            ),
            column![
                text("LOKmaus-Anzeigename")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(font::bold_font()),

                text_input("LOKmaus-Name", lokbuch.state.lok_maus_name_input.as_str())
                .id("new-lok-short-name")
                .on_input_maybe(
                    if lokbuch.state.has_decoder_input {
                        Some(Message::LokMausNameInputChanged)
                    }
                    else {
                        None
                    }
                 )
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            ]
        ].spacing(20).padding(20);

    let lower_row = row![
            column![
                text("Hersteller")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(font::bold_font()),

                text_input("Hersteller", lokbuch.state.producer_input.as_str())
                .id("new-lok-producer")
                .on_input(Message::ProducerInputChanged)
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            ],
            column!(
                text("Bahnverwaltung")
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(font::bold_font()),

                text_input("Bahnverwaltung", lokbuch.state.management_input.as_str())
                .id("new-lok-management")
                .on_input(Message::ManagementInputChanged)
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            )
        ].spacing(20).padding(20);

    let add_button = button(text("Speichern").size(ui::HEADING_TEXT_SIZE))
        .on_press(message_on_finish)
        .padding(15);

    let cancel_button = button(text("Abbrechen").size(ui::HEADING_TEXT_SIZE))
        .on_press(Message::Cancel)
        .padding(15);

    column![
            header,
            column![
                column![
                    upper_row,
                ],
                column![
                    vertical_space(),
                    center_row,
                    vertical_space(),
                    lower_row,
                    vertical_space()
                ]
            ].height(Fill),
            row![
                horizontal_space(),
                add_button,
                horizontal_space(),
                cancel_button,
                horizontal_space(),
            ],
        ].width(Fill).into()
}
