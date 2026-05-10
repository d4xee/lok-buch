use crate::app::backend::database::lok::Lok;
use crate::app::Message;
use iced::Task;
use std::u32;

/// The State holds data for a session.
/// Does not hold persistent data.
#[derive(Clone)]
pub struct State {
    pub name_input: String,
    pub address_input: i32,
    pub lok_maus_name_input: String,
    pub manufacturer_input: String,
    pub management_input: String,
    pub has_decoder: bool,
    pub image_path_input: String,
    pub search_input: String,
    pub selected_lok_id: Option<u32>,
}

impl State {
    /// Clears all internal state variables.
    /// These are usually the user inputs.
    pub fn clear(&mut self) {
        self.name_input.clear();
        self.address_input = 0;
        self.lok_maus_name_input.clear();
        self.manufacturer_input.clear();
        self.management_input.clear();
        self.has_decoder = false;
        self.image_path_input.clear();
        self.search_input.clear();
        self.selected_lok_id = None;
    }

    /// Returns a new Lok instance built from the inputted data.
    pub fn get_lok_from_current_state(&self) -> Lok {
        Lok::new_from_raw_data(
            self.name_input.clone(),
            if self.has_decoder {
                self.address_input as i32
            } else {
                -1
            },
            if self.has_decoder {
                self.lok_maus_name_input.clone().to_string().to_uppercase()
            } else {
                "".to_string()
            },
            self.manufacturer_input.clone(),
            self.management_input.clone(),
            self.has_decoder.clone(),
            self.image_path_input.clone(),
        )
    }

    /// Defines the correct inputs for certain fields.
    /// Validates the inputted data.
    pub fn validate(&self) -> Result<(), Task<Message>> {
        if self.name_input.is_empty() {
            let res = rfd::AsyncMessageDialog::new()
                .set_title(t!("state.input_error"))
                .set_description(t!("state.name_must_not_empty"))
                .set_buttons(rfd::MessageButtons::Ok);

            return Err(Task::perform(res.show(), Message::InputFailure));
        }

        if self.has_decoder {
            if self.lok_maus_name_input.len() > 5 {
                let res = rfd::AsyncMessageDialog::new()
                    .set_title(t!("state.input_error"))
                    .set_description(t!("state.lm_name_too_long"))
                    .set_buttons(rfd::MessageButtons::Ok);

                return Err(Task::perform(res.show(), Message::InputFailure));
            }
        }

        Ok(())
    }

    /// Updates the state depending on the message.
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NameInputChanged(name) => {
                self.name_input = name;
            }
            Message::AddressInputChanged(address) => {
                self.address_input = address;
            }
            Message::LokMausNameInputChanged(name) => {
                self.lok_maus_name_input = name;
            }
            Message::ManufacturerInputChanged(producer) => {
                self.manufacturer_input = producer;
            }
            Message::ManagementInputChanged(management) => {
                self.management_input = management;
            }
            Message::HasDecoderInputChanged(_) => {
                self.has_decoder = !self.has_decoder;
            }
            _ => {}
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            name_input: String::default(),
            address_input: 0,
            lok_maus_name_input: String::default(),
            manufacturer_input: String::default(),
            management_input: String::default(),
            has_decoder: false,
            image_path_input: String::default(),
            search_input: String::default(),
            selected_lok_id: None,
        }
    }
}