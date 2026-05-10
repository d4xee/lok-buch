use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::state::State;
use crate::app::ui;
use crate::app::ui::widgets::{button_decorations, page_layout, preview_widget};
use crate::app::ui::SvgIcon;
use crate::app::Lokbuch;
use async_std::task;
use iced::widget::operation::focus;
use iced::widget::{button, column, container, keyed_column, row, scrollable, space, text, text_input};
use iced::{Center, Element, Fill, FillPortion, Task};

pub struct HomePage;

impl Page for HomePage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        match message {
            Message::Add => {
                lokbuch.change_page_to(Pages::Add);
                lokbuch.state.clear();
                return focus("new-lok-name");
            }

            Message::SearchInputChanged(search_input) => {
                lokbuch.state.search_input = search_input.clone();

                lokbuch.lok_resource_manager.search_and_store_previews_containing(search_input.to_lowercase());
            }

            Message::ShowLok(id) => {
                let lok = task::block_on(lokbuch.lok_resource_manager.get_lok(id)).expect("lok not found");

                lokbuch.state = State::create_state_from_id_and_lok(id, &lok);

                lokbuch.change_page_to(Pages::Show);
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
        let num_of_loks = lokbuch.lok_resource_manager.number_of_loks();
        let mut previews = lokbuch.lok_resource_manager.get_all_previews().clone();

        let input_search = text_input(t!("home.search").to_string().as_str(), lokbuch.state.search_input.as_str())
            .id("lok-search")
            .on_input(Message::SearchInputChanged)
            .padding(15)
            .size(ui::HEADING_TEXT_SIZE)
            .align_x(Center);

        let add_button = button(button_decorations(t!("home.new_loco").to_string(), SvgIcon::Plus))
            .on_press(Message::Add)
            .padding(15)
            .width(Fill);

        let text_row = row![
                    space::horizontal()
                    .width(10),

                    text(t!("home.address"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    space::horizontal(),

                    text(t!("home.lm_name"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    space::horizontal(),

                    text(t!("home.name"))
                    .size(ui::HEADING_TEXT_SIZE)
                    .font(ui::font::bold_font()),

                    space::horizontal(),

                    space::horizontal(),

                    space::horizontal()
                ];

        let loks =
            if lokbuch.state.search_input.is_empty() {
                keyed_column(
                    previews.into_iter().map(move |item| {
                        let preview = item.clone();
                        (0, iced::widget::column!(
                        button(preview_widget(preview.clone()))
                        .style(button::text)
                        .on_press_with(move || {
                            Message::ShowLok(item.clone().get_id())
                        }),
                        space::vertical()
                            .height(10))
                            .into())
                    })
                ).width(Fill)
            } else {
                keyed_column(
                    lokbuch.lok_resource_manager.get_search_results()
                        .into_iter().map(move |item| {
                        let preview = item.clone();
                        (0, iced::widget::column!(
                        button(preview_widget(preview.clone()))
                        .style(button::text)
                        .on_press_with(move || {
                            Message::ShowLok(item.clone().get_id())
                        }),
                        space::vertical()
                            .height(10))
                            .into())
                    })
                ).width(Fill)
            };


        let content = container(
            column!(
                input_search,
                text_row,
                scrollable(container(loks))
            ).align_x(Center).spacing(20).width(FillPortion(7))
        ).padding(10);

        page_layout(t!("home.locos_available", num=num_of_loks).to_string(), column![add_button], content, false)
    }
}