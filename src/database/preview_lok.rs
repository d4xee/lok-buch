use std::cmp::Ordering;
use sqlx::{Pool, Sqlite};
use crate::ui;

#[derive(Clone, Debug, Eq, Hash)]
pub struct PreviewLok {
    id: i32,
    address: Option<i32>,
    name: Option<String>,
    lokmaus_name: Option<String>,
}

#[derive(sqlx::FromRow)]
struct PreviewData {
    id: i32,
    address: i32,
    name: String,
    lokmaus_name: String,
}

async fn get_all_previews_data(db: &Pool<Sqlite>) -> Vec<PreviewData> {
    sqlx::query_as("select id, address, name, lokmaus_name from loks").fetch_all(db).await.unwrap()
}

pub async fn get_all_previews(db: &Pool<Sqlite>) -> Vec<PreviewLok> {
    let data = get_all_previews_data(db).await;

    data.iter().map(|raw_preview| {
        PreviewLok::new_from_raw_preview_data(raw_preview)
    }).collect()
}

impl PreviewLok {
    pub(crate) fn new(id: i32, address: Option<i32>, name: Option<String>, lokmaus_name: Option<String>) -> Self {
        Self {
            id,
            address,
            name,
            lokmaus_name,
        }
    }

    fn new_from_raw_preview_data(data: &PreviewData) -> Self {
        PreviewLok::new(
            data.id,
            if data.address < 0 { None } else { Some(data.address) },
            if data.name.is_empty() { None } else { Some(data.name.clone()) },
            if data.lokmaus_name.is_empty() { None  } else { Some(data.lokmaus_name.clone()) }
        )
    }

    pub fn get_id(&self) -> i32 {
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
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_name_pretty(&self) -> String {
        if let Some(name) = self.name.clone() {
            name
        }
        else {
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
        }
    }

    pub fn get_lokmaus_name_pretty(&self) -> String {
        if let Some(lokmaus_name) = self.lokmaus_name.clone() {
            lokmaus_name
        }
        else {
            ui::NO_DATA_AVAILABLE_TEXT.to_string()
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