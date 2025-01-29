use crate::app::Message;
use crate::backend::database::lok::Lok;
use iced::Task;

/// The State holds all data that is needed during runtime.
#[derive(Clone)]
pub struct State {
    pub name_input: String,
    pub address_input: String,
    pub lok_maus_name_input: String,
    pub producer_input: String,
    pub management_input: String,
    pub has_decoder_input: bool,
    pub image_path_input: String,
    pub search_input: String,
    pub selected_lok_id: Option<u32>,
}

impl State {
    /// Clears all internal state variables.
    /// These are usually the user inputs.
    pub fn clear(&mut self) {
        self.name_input.clear();
        self.address_input.clear();
        self.lok_maus_name_input.clear();
        self.producer_input.clear();
        self.management_input.clear();
        self.has_decoder_input = false;
        self.image_path_input.clear();
        self.search_input.clear();
        self.selected_lok_id = None;
    }

    /// Returns a new Lok instance built from the inputted data.
    pub fn get_lok_from_current_state(&self) -> Lok {
        Lok::new_from_raw_data(
            self.name_input.clone(),
            if self.has_decoder_input {
                self.address_input.clone().parse::<i32>().unwrap_or(-1)
            } else {
                -1
            },
            if self.has_decoder_input {
                self.lok_maus_name_input.clone().to_string().to_uppercase()
            } else {
                "".to_string()
            },
            self.producer_input.clone(),
            self.management_input.clone(),
            self.has_decoder_input.clone(),
            self.image_path_input.clone(),
        )
    }

    /// Defines the correct inputs for certain fields.
    /// Validates the inputted data.
    pub fn validate(&self) -> Result<(), Task<Message>> {
        if self.name_input.is_empty() {
            let res = rfd::AsyncMessageDialog::new()
                .set_title("Eingabefehler")
                .set_description("Bezeichnung darf nicht leer sein!")
                .set_buttons(rfd::MessageButtons::Ok);

            return Err(Task::perform(res.show(), Message::InputFailure));
        }

        if self.has_decoder_input {
            if !self.address_input.clone().is_empty() {
                let address = self.address_input.clone().parse::<i32>();
                if address.is_ok() {
                    let address = address.unwrap();

                    if address <= 0 {
                        let res = rfd::AsyncMessageDialog::new()
                            .set_title("Eingabefehler")
                            .set_description("Adresse muss eine Zahl größer als 0 sein!")
                            .set_buttons(rfd::MessageButtons::Ok);

                        return Err(Task::perform(res.show(), Message::InputFailure));
                    }
                } else {
                    let res = rfd::AsyncMessageDialog::new()
                        .set_title("Eingabefehler")
                        .set_description("Adresse muss eine Zahl sein!")
                        .set_buttons(rfd::MessageButtons::Ok);

                    return Err(Task::perform(res.show(), Message::InputFailure));
                }
            }

            if self.lok_maus_name_input.len() > 5 {
                let res = rfd::AsyncMessageDialog::new()
                    .set_title("Eingabefehler")
                    .set_description("Der LOKmaus-Anzeigename darf nicht länger als 5 Zeichen sein!")
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
            Message::ProducerInputChanged(producer) => {
                self.producer_input = producer;
            }
            Message::ManagementInputChanged(management) => {
                self.management_input = management;
            }
            Message::HasDecoderInputChanged(_) => {
                self.has_decoder_input = !self.has_decoder_input;
            }
            _ => {}
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            name_input: String::default(),
            address_input: String::default(),
            lok_maus_name_input: String::default(),
            producer_input: String::default(),
            management_input: String::default(),
            has_decoder_input: false,
            image_path_input: String::default(),
            search_input: String::default(),
            selected_lok_id: None,
        }
    }
}