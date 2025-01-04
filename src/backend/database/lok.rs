use crate::database::preview_lok::PreviewLok;
use crate::ui;

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Lok {
    pub name: String,
    pub address: Option<i32>,
    pub lokmaus_name: Option<String>,
    pub producer: Option<String>,
    pub management: Option<String>,
}

#[derive(sqlx::FromRow, Clone, Debug, Default)]
pub struct RawLokData {
    id: i32,
    name: String,
    address: i32,
    lokmaus_name: String,
    producer: String,
    management: String,
}

impl Lok {
    fn new(
        name: String,
        address: Option<i32>,
        lokmaus_name: Option<String>,
        producer: Option<String>,
        management: Option<String>) -> Lok {
        Lok {
            name,
            address,
            lokmaus_name,
            producer,
            management,
        }
    }

    pub fn new_from_raw_lok_data(raw_lok_data: &RawLokData) -> Lok {
        Lok::new(
            raw_lok_data.name.clone(),
            if raw_lok_data.address < 0 { None } else { Some(raw_lok_data.address) },
            if raw_lok_data.lokmaus_name.is_empty() { None } else { Some(raw_lok_data.lokmaus_name.clone()) },
            if raw_lok_data.producer.is_empty() { None } else { Some(raw_lok_data.producer.clone()) },
            if raw_lok_data.management.is_empty() { None } else { Some(raw_lok_data.management.clone()) },
        )
    }

    pub fn new_from_raw_data(name: String, address: i32, lokmaus_name: String, producer: String, management: String) -> Lok {
        Lok::new_from_raw_lok_data(&RawLokData {
            name,
            address,
            lokmaus_name,
            producer,
            management,
            ..Default::default()
        })
    }

    fn as_raw_lok_data(&self) -> RawLokData {
        RawLokData {
            name: self.name.clone(),
            address: if let Some(adress) = self.address {
                adress
            } else { -1 },
            lokmaus_name: if let Some(lokmaus_name) = self.lokmaus_name.clone() {
                lokmaus_name
            } else { String::new() },
            producer: if let Some(producer) = self.producer.clone() {
                producer
            } else { String::new() },
            management: if let Some(management) = self.management.clone() {
                management
            } else { String::new() },
            ..RawLokData::default()
        }
    }

    pub fn as_preview_lok(&self, id: u32) -> PreviewLok {
        PreviewLok::new(id, self.address.clone(), Some(self.name.clone()), self.lokmaus_name.clone())
    }

    pub fn get_address_pretty(&self) -> String {
        if let Some(address) = self.address {
            address.to_string()
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

    pub fn get_producer_pretty(&self) -> String {
        if let Some(producer) = self.producer.clone() {
            producer
        } else {
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_management_pretty(&self) -> String {
        if let Some(management) = self.management.clone() {
            management
        } else {
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }
}
