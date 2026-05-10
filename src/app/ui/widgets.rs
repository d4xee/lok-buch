use crate::app::backend::database::preview_lok::PreviewLok;
use crate::app::message::Message;
use crate::app::ui::{font, SvgIcon, VIEW_NAME_TEXT_SIZE, VIEW_TITLE_TEXT_SIZE};
use crate::app::{ui, Lokbuch};
use iced::widget::{button, checkbox, column, container, image, row, space, svg, text, text_input, Container};
use iced::{Center, ContentFit, Element, Fill, FillPortion, Left};
use iced_aw::number_input;

pub fn header<'a>(name: String) -> Element<'a, Message> {
    container(
        iced::widget::column!(
        text("LOKBUCH")
            .width(Fill)
            .size(VIEW_TITLE_TEXT_SIZE)
            .color([0.3, 0.3, 0.5])
            .align_x(Center)
            .font(font::bold_font()),

        text(name)
            .width(Fill)
            .size(VIEW_NAME_TEXT_SIZE)
            .color([0.5, 0.5, 0.5])
            .align_x(Center),

        space::vertical()
        .height(15)
    )
            .spacing(20)
            .width(Fill)
            .align_x(Center)
    ).style(container::bordered_box).into()
}

/// Returns an inputted PreviewLok as a custom widget.
/// This is used for the main page.
pub fn preview_widget<'a>(preview_data: PreviewLok) -> Container<'a, Message> {
    let preview_id = preview_data.get_id();

    let button_row = row![
        space::horizontal(),
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
            space::horizontal(),
        ],

        row![
            text!("{}", preview_data.get_lokmaus_name_pretty()),
            space::horizontal(),
        ],

        row![
            text!("{}", preview_data.get_name_pretty()),
            space::horizontal(),
        ],

        space::horizontal(),
        button_row
        ])
        .padding(10)
        .style(container::rounded_box)
        .width(Fill)
}

/// Layouts the input mask for adding and editing a Lok.
/// The message on finish is emitted when the save button was pressed.
pub fn lok_data_input_mask(lokbuch: &Lokbuch, header_text: String, message_on_finish: Message) -> Element<Message> {
    let upper_row = row![
            button(image("res/images/blue.png").width(400).content_fit(ContentFit::Cover)).style(button::text),
            column![
                column!(
                text(t!("ui.name"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(font::bold_font()),

                text_input(t!("ui.name").to_string().as_str(), lokbuch.state.name_input.as_str())
                    .id("new-lok-name")
                    .on_input(Message::NameInputChanged)
                    .padding(15)
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left),
                ),
                space::vertical(),

                column!(
                text(t!("ui.analogue_digital"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left)
                    .font(font::bold_font()),

                checkbox(!lokbuch.state.has_decoder)
                    .label(t!("ui.analogue"))
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(ui::HEADING_TEXT_SIZE),

                checkbox(lokbuch.state.has_decoder)
                    .label(t!("ui.digital"))
                    .on_toggle(Message::HasDecoderInputChanged)
                    .text_size(ui::HEADING_TEXT_SIZE),
                ),
                space::vertical(),
            ]
        ].spacing(20).padding(20);

    let center_row = row![
            column!(
            text(t!("ui.address"))
            .size(ui::HEADING_TEXT_SIZE)
            .align_x(Left)
            .font(font::bold_font()),
            
            if lokbuch.state.has_decoder {
                container(
                    number_input(&lokbuch.state.address_input, 0..i32::MAX, Message::AddressInputChanged)
                    .padding(15)
                    .line_height(ui::HEADING_TEXT_SIZE)
                    .width(Fill)
                )
            } else {
                container(
                    text_input(t!("ui.address").to_string().as_str(), "")
                    .padding(15)
                    .size(ui::HEADING_TEXT_SIZE)
                    .align_x(Left)
                )
            },
            ),
            column![
                text(t!("ui.lm_name"))
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(font::bold_font()),

                text_input(t!("ui.lm_name").to_string().as_str(), lokbuch.state.lok_maus_name_input.as_str())
                .id("new-lok-short-name")
                .on_input_maybe(
                    if lokbuch.state.has_decoder {
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
                text(t!("ui.producer"))
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(font::bold_font()),

                text_input(t!("ui.producer").to_string().as_str(), lokbuch.state.producer_input.as_str())
                .id("new-lok-producer")
                .on_input(Message::ProducerInputChanged)
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            ],
            column!(
                text(t!("ui.management"))
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left)
                .font(font::bold_font()),

                text_input(t!("ui.management").to_string().as_str(), lokbuch.state.management_input.as_str())
                .id("new-lok-management")
                .on_input(Message::ManagementInputChanged)
                .padding(15)
                .size(ui::HEADING_TEXT_SIZE)
                .align_x(Left),
            )
        ].spacing(20).padding(20);

    let add_button = button(text(t!("ui.save")))
        .on_press(message_on_finish)
        .padding(15)
        .width(Fill);

    let content = container(
        column![
            column![
                column![
                    upper_row,
                ],
                column![
                    space::vertical(),
                    center_row,
                    space::vertical(),
                    lower_row,
                    space::vertical(),
                ]
            ].height(Fill),
        ].width(Fill)
    );

    page_layout(header_text, column![add_button], content, true)
}

pub fn sidebar(buttons: iced::widget::Column<Message>, has_cancel_button: bool) -> Element<Message> {
    let side_column = column![];

    let side_column = side_column.push(if has_cancel_button {
        column![
            button(button_decorations(t!("ui.cancel").to_string(), SvgIcon::Back))
            .on_press(Message::Cancel)
            .padding(13)
            .width(Fill),
            buttons.width(Fill).spacing(20)
        ].spacing(20)
    } else {
        buttons.width(Fill).spacing(20)
    });

    container(
        column![
            side_column,
            space::vertical().height(Fill),
            button(button_decorations(t!("ui.settings").to_string(), SvgIcon::Gear))
            .on_press(Message::Settings)
            .width(Fill)
            .padding(15),
        ].spacing(20)
    )
        .padding(15)
        .style(container::rounded_box)
        .height(Fill)
        .width(FillPortion(1))
        .into()
}

pub fn page_layout<'a>(title: String, sidebar_buttons: iced::widget::Column<'a, Message>, content: Container<'a, Message>, has_cancel_button: bool) -> Element<'a, Message> {
    column![
        header(title),
        row![
            content.width(FillPortion(7)),
            sidebar(sidebar_buttons, has_cancel_button),
        ]
    ].into()
}

pub fn button_decorations<'a>(text_: String, icon: SvgIcon) -> Element<'a, Message> {
    row![
        svg(icon.get_file_path()).height(ui::SVG_ICON_HEIGHT).width(FillPortion(1)).content_fit(ContentFit::Cover),
        text(text_).width(FillPortion(5)).align_y(Center),
    ].spacing(10).into()
}