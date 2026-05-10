use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::state::State;
use crate::app::ui;
use crate::app::ui::widgets::{button_decorations, page_layout};
use crate::app::ui::SvgIcon;
use crate::app::Lokbuch;
use async_std::task;
use iced::widget::operation::focus;
use iced::widget::{button, container, image, row, space, text};
use iced::{ContentFit, Element, Fill, Task};

pub struct ShowPage;

impl Page for ShowPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        match message {
            Message::Add => {
                lokbuch.change_page_to(Pages::Add);
                return focus("new-lok-name");
            }

            Message::Cancel => {
                lokbuch.state.clear();
                lokbuch.change_page_to(Pages::Home);
            }

            Message::Edit(id) => {
                let lok = task::block_on(lokbuch.lok_resource_manager.get_lok(id)).expect("Lok does not exist!"); // TODO async edit

                lokbuch.state = State::create_state_from_id_and_lok(id, &lok);

                lokbuch.change_page_to(Pages::Edit);
                return focus("new-lok-name");
            }

            Message::Remove(id) => {
                task::block_on(lokbuch.lok_resource_manager.remove_lok(id)); // TODO async remove

                lokbuch.change_page_to(Pages::Home);
            }

            _ => {}
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        let mut lrm = lokbuch.lok_resource_manager.clone();
        let lok = task::block_on(lrm.get_lok(lokbuch.state.selected_lok_id.unwrap())).expect("lok not found"); // TODO async show

        let left_column = iced::widget::column!(
                    image(lokbuch.state.get_current_lok_image_path())
                    .width(400)
                    .height(200)
                    .content_fit(ContentFit::Cover),

                    space::vertical(),

                    text(t!("show.address"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_address_pretty())
                    .size(ui::HEADING_TEXT_SIZE),

                    space::vertical(),

                    text(t!("show.producer"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_producer_pretty())
                    .size(ui::HEADING_TEXT_SIZE),
                ).spacing(10);

        let right_column = iced::widget::column![
                    text(t!("show.name"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.name)
                    .size(ui::HEADING_TEXT_SIZE),

                    space::vertical(),

                    text(t!("show.lm_name"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_lokmaus_name_pretty())
                    .size(ui::HEADING_TEXT_SIZE),

                    space::vertical(),

                    text(t!("show.management"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    text!("{}", lok.get_management_pretty())
                    .size(ui::HEADING_TEXT_SIZE)
                ].spacing(10);

        let edit_button = button(button_decorations(t!("show.edit").to_string(), SvgIcon::Edit))
            .on_press_with(move || {
                Message::Edit(lokbuch.state.selected_lok_id.clone().unwrap())
            })
            .padding(15)
            .width(Fill);

        let remove_button = button(button_decorations(t!("show.delete").to_string(), SvgIcon::Trash))
            .on_press_with(move || {
                Message::Remove(lokbuch.state.selected_lok_id.clone().unwrap())
            })
            .style(button::danger)
            .padding(15)
            .width(Fill);

        let content = container(
            iced::widget::column![
                row![
                    space::horizontal(),
                    left_column,
                    space::horizontal(),
                    right_column,
                    space::horizontal(),
                ].width(Fill).height(Fill)
            ].width(Fill)
        );

        page_layout(format!("{}", lok.name), iced::widget::column![
            edit_button,
            remove_button,
        ], content, true)
    }
}