use crate::frontend;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash)]
pub struct PreviewLok {
    id: u32,
    address: Option<i32>,
    name: Option<String>,
    lokmaus_name: Option<String>,
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
        Self {
            id,
            address,
            name,
            lokmaus_name,
        }
    }

    pub fn new_from_raw_preview_data(data: &PreviewData) -> Self {
        PreviewLok::new(
            data.id as u32,
            if data.address < 0 { None } else { Some(data.address) },
            if data.name.is_empty() { None } else { Some(data.name.clone()) },
            if data.lokmaus_name.is_empty() { None  } else { Some(data.lokmaus_name.clone()) }
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
        if let Some(address) = self.address.clone() {
            address.to_string()
        }
        else {
            frontend::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_name_pretty(&self) -> String {
        if let Some(name) = self.name.clone() {
            name
        }
        else {
            frontend::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_lokmaus_name_pretty(&self) -> String {
        if let Some(lokmaus_name) = self.lokmaus_name.clone() {
            lokmaus_name
        }
        else {
            frontend::NO_DATA_AVAILABLE_TEXT.to_string()
        }
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