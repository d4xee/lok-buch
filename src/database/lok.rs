use sqlx::{Pool, Row, Sqlite};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq)]
pub struct Lok {
    pub name: String,
    pub address: i32,
    pub lokmaus_name: String,
    pub producer: String,
    pub management: String,
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

async fn insert_raw_lok(db: &Pool<Sqlite>, lok: RawLokData) {
    let result = sqlx::query("INSERT INTO loks (name, address, lokmaus_name, producer, management) VALUES (?, ?, ?, ?, ?)")
        .bind(lok.name)
        .bind(lok.address)
        .bind(lok.lokmaus_name)
        .bind(lok.producer)
        .bind(lok.management)
        .execute(db)
        .await
        .unwrap();

    println!("Inserted new lok: {:?}", result)
}

async fn get_all_raw_loks(db: &Pool<Sqlite>) -> Vec<RawLokData> {
    sqlx::query_as("select * from loks").fetch_all(db).await.unwrap()
}

pub async fn get_all_loks(db: &Pool<Sqlite>) -> Vec<Lok> {
    let raw_loks = get_all_raw_loks(db).await;

    raw_loks.iter().map(|lok| Lok {
        name: lok.name.clone(),
        address: lok.address,
        lokmaus_name: lok.lokmaus_name.clone(),
        producer: lok.producer.clone(),
        management: lok.management.clone(),
    }).collect()
}

async fn delete_raw_lok(db: &Pool<Sqlite>, mut lok: RawLokData) {
    lok.get_id(db).await;

    let result = sqlx::query("DELETE FROM loks WHERE id = ?")
        .bind(lok.id)
        .execute(db)
        .await.unwrap();

    println!("Deleted lok: {:?}", result)
}

async fn update_raw_lok(db: &Pool<Sqlite>, mut old_lok: RawLokData, new_lok: RawLokData) {
    old_lok.get_id(db).await;

    let result = sqlx::query("UPDATE loks SET address = ?, name = ?, lokmaus_name = ?, producer = ?, management = ? WHERE id = ?;")
        .bind(new_lok.address)
        .bind(new_lok.name)
        .bind(new_lok.lokmaus_name)
        .bind(new_lok.producer)
        .bind(new_lok.management)
        .bind(old_lok.id)
        .execute(db)
        .await.unwrap();

    println!("updated lok: {:?}", result)
}

impl Lok {
    fn as_raw_lok(&self) -> RawLokData {
        RawLokData {
            name: self.name.clone(),
            address: self.address,
            lokmaus_name: self.lokmaus_name.clone(),
            producer: self.producer.clone(),
            management: self.management.clone(),
            ..RawLokData::default()
        }
    }

    pub async fn save(&self, db: &Pool<Sqlite>) {
        insert_raw_lok(db, self.as_raw_lok()).await;
    }

    pub async fn delete(&self, db: &Pool<Sqlite>) {
        delete_raw_lok(db, self.as_raw_lok()).await;
    }

    pub async fn update(&self, db: &Pool<Sqlite>, updated_lok: Lok) {
        update_raw_lok(db, self.as_raw_lok(), updated_lok.as_raw_lok()).await;
    }
}

impl PartialEq<Self> for Lok {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl PartialOrd<Self> for Lok {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lok {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.cmp(&other.address)
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