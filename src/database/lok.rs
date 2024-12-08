use crate::ui;
use futures::join;
use sqlx::{Pool, Row, Sqlite};
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::database::preview_lok::PreviewLok;

#[derive(Clone, Debug, Eq, Hash)]
pub struct Lok {
    pub name: String,
    pub address: Option<i32>,
    pub lokmaus_name: Option<String>,
    pub producer: Option<String>,
    pub management: Option<String>,
}

#[derive(sqlx::FromRow, Clone, Debug, Default)]
struct RawLokData {
    id: i32,
    name: String,
    address: i32,
    lokmaus_name: String,
    producer: String,
    management: String,
}

async fn insert_raw_lok(db: &Pool<Sqlite>, lok: RawLokData) -> i32 {
    let result = sqlx::query("INSERT INTO loks (name, address, lokmaus_name, producer, management) VALUES (?, ?, ?, ?, ?)")
        .bind(lok.name)
        .bind(lok.address)
        .bind(lok.lokmaus_name)
        .bind(lok.producer)
        .bind(lok.management)
        .execute(db)
        .await
        .unwrap();

    println!("Inserted new lok: {:?}", result);

    result.last_insert_rowid() as i32
}

async fn get_all_raw_loks(db: &Pool<Sqlite>) -> Vec<RawLokData> {
    sqlx::query_as("select * from loks").fetch_all(db).await.unwrap()
}

async fn get_all_ids(db: &Pool<Sqlite>) -> Vec<i32> {
    sqlx::query("select id from loks").fetch_all(db).await.unwrap().iter().map(|item| {
        item.get("id")
    }).collect()
}

pub async fn get_all_loks_new(db: &Pool<Sqlite>) -> HashMap<i32, Lok> {
    let id_task = get_all_ids(db);
    let raw_loks_task = get_all_raw_loks(db);

    let data = join!(id_task, raw_loks_task);

    let ids = data.0;
    let raw_loks = data.1;

    let raw_loks: Vec<Lok> = raw_loks.iter().map(|data| {
        Lok::new_from_raw_lok_data(data)
    }).collect();

    ids.into_iter().zip(raw_loks.into_iter()).collect()
}

pub async fn get_all_loks(db: &Pool<Sqlite>) -> Vec<Lok> {
    let raw_loks = get_all_raw_loks(db).await;

    raw_loks.iter().map(|data| {
        Lok::new_from_raw_lok_data(data)
    }).collect()
}

async fn delete_raw_lok_by_id(db: &Pool<Sqlite>, id: i32) {
    let result = sqlx::query("DELETE FROM loks WHERE id = ?")
        .bind(id)
        .execute(db)
        .await.unwrap();

    println!("Deleted lok: {:?}", result)
}

async fn update_raw_lok(db: &Pool<Sqlite>, old_lok_id: i32, new_lok: RawLokData) {
    let result = sqlx::query("UPDATE loks SET address = ?, name = ?, lokmaus_name = ?, producer = ?, management = ? WHERE id = ?;")
        .bind(new_lok.address)
        .bind(new_lok.name)
        .bind(new_lok.lokmaus_name)
        .bind(new_lok.producer)
        .bind(new_lok.management)
        .bind(old_lok_id)
        .execute(db)
        .await.unwrap();

    println!("updated lok: {:?}", result)
}

async fn get_raw_lok_by_id(db: &Pool<Sqlite>, id: i32) -> RawLokData {
    sqlx::query_as("select * from loks where id = ? limit 1")
        .bind(id)
        .fetch_one(db)
        .await.unwrap()
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

    fn new_from_raw_lok_data(raw_lok_data: &RawLokData) -> Lok {
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

    pub fn as_preview_lok(&self, id: i32) -> PreviewLok {
        PreviewLok::new(id, self.address.clone(), Some(self.name.clone()), self.lokmaus_name.clone())
    }

    pub async fn save(&self, db: &Pool<Sqlite>) -> i32 {
        insert_raw_lok(db, self.as_raw_lok_data()).await
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

    pub async fn get_lok_by_id(db: &Pool<Sqlite>, id: i32) -> Lok {
        Lok::new_from_raw_lok_data(&get_raw_lok_by_id(db, id).await)
    }

    pub async fn delete_lok_by_id(db: &Pool<Sqlite>, id: i32) {
        delete_raw_lok_by_id(db, id).await;
    }

    pub async fn update_lok_by_id(db: &Pool<Sqlite>, id: i32, updated_lok: Lok) {
        update_raw_lok(db, id, updated_lok.as_raw_lok_data()).await;
    }
}

impl PartialEq<Self> for Lok {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.name == other.name
    }
}

impl PartialOrd<Self> for Lok {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lok {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.address != None || other.address != None {
            self.address.cmp(&other.address)
        } else { self.name.cmp(&other.name) }
    }
}

impl RawLokData {
    async fn get_id(&mut self, db: &Pool<Sqlite>) {
        self.id = sqlx::query("SELECT id FROM loks WHERE address = ? AND name = ? AND lokmaus_name = ?")
            .bind(self.address)
            .bind(self.name.clone())
            .bind(self.lokmaus_name.clone())
            .fetch_one(db)
            .await
            .unwrap()
            .get("id");
    }
}