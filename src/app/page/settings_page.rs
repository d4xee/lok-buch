use crate::app::message::Message;
use crate::app::page::{Page, Pages};
use crate::app::settings::languages::Languages;
use crate::app::ui::widgets::page_layout;
use crate::app::Lokbuch;
use iced::widget::{column, container, text};
use iced::{Element, Task};
use iced_aw::SelectionList;
use rust_i18n::set_locale;

const LANGUAGES: &[&str] = &["en", "de"];

pub struct SettingsPage;

impl Page for SettingsPage {
    fn update(&self, lokbuch: &mut Lokbuch, message: Message) -> Task<Message> {
        match message {
            Message::Cancel => {
                lokbuch.change_page_to(Pages::Home)
            }

            Message::LanguageSelected(index, language) => {
                println!("{} {}", index, language);
                lokbuch.settings.language = language.short_language_code();
                set_locale(&language.short_language_code());
            }
            _ => {}
        }
        Task::none()
    }

    fn view<'a>(&self, lokbuch: &'a Lokbuch) -> Element<'a, Message> {
        let content = container(
            column![
                text(t!("settings.language")),
                SelectionList::new(&Languages::ALL, Message::LanguageSelected),
            ]
        );

        page_layout(t!("settings.settings").to_string(), iced::widget::Column::new(), content, true)
    }
}