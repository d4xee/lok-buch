use crate::app::ui;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash)]
pub struct PreviewLok {
    id: u32,
    address: Option<i32>,
    name: Option<String>,
    lokmaus_name: Option<String>,
    search_text: String,
}

#[derive(sqlx::FromRow)]
pub struct PreviewData {
    id: i32,
    address: i32,
    name: String,
    lokmaus_name: String,
}

impl PreviewLok {
    pub(crate) fn new(id: u32, address: Option<i32>, name: Option<String>, lokmaus_name: Option<String>) -> Self {
        let address_text = address.unwrap_or(-1);
        let address_text = if address_text < 0 { "".to_string() } else { address_text.to_string() };
        Self {
            id,
            address,
            name: name.clone(),
            lokmaus_name: lokmaus_name.clone(),
            search_text: String::from(
                format!(
                    "{} {} {}",
                    address_text,
                    name.unwrap_or("".to_string()),
                    lokmaus_name.unwrap_or("".to_string())
                )
            ),
        }
    }

    pub fn new_from_raw_preview_data(data: &PreviewData) -> Self {
        PreviewLok::new(
            data.id as u32,
            if data.address < 0 { None } else { Some(data.address) },
            if data.name.is_empty() { None } else { Some(data.name.clone()) },
            if data.lokmaus_name.is_empty() { None } else { Some(data.lokmaus_name.clone()) },
        )
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_address(&self) -> Option<i32> {
        self.address.clone()
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn get_lokmaus_name(&self) -> Option<String> {
        self.lokmaus_name.clone()
    }

    pub fn get_address_pretty(&self) -> String {
        if let Some(address) = self.address {
            if address < 0 {
                ui::NO_DATA_AVAILABLE_TEXT.to_string()
            } else {
                address.to_string()
            }
        } else {
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_name_pretty(&self) -> String {
        if let Some(name) = self.name.clone() {
            name
        } else {
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_lokmaus_name_pretty(&self) -> String {
        if let Some(lokmaus_name) = self.lokmaus_name.clone() {
            lokmaus_name
        } else {
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_search_string(&self) -> String {
        self.search_text.clone()
    }
}

impl PartialEq<Self> for PreviewLok {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.name == other.name
    }
}

impl PartialOrd<Self> for PreviewLok {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PreviewLok {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.address != None || other.address != None {
            self.address.cmp(&other.address)
        } else { self.name.cmp(&other.name) }
    }
}