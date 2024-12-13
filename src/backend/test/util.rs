use crate::database::sqlite_db::SQLiteDB;
use crate::database::Database;
use async_std::task;
use std::time::Duration;

pub const AFTER_DELETE_WAIT_TIME: u64 = 200;
pub const TEST_DB_LOCATION: &str = "sqlite://test/test.db";

pub fn remove_test_db(index: i32) {
    match std::fs::remove_file(format!("test/test{}.db", index)) {
        Ok(_) => { println!("Removed test db"); }
        Err(_) => { println!("Failed to remove test db"); }
    }
}

pub fn build_db(index: i32) -> SQLiteDB {
    remove_test_db(index);
    std::thread::sleep(Duration::from_millis(AFTER_DELETE_WAIT_TIME));
    task::block_on(SQLiteDB::build(format!("sqlite://test/test{}.db", index).as_str())).unwrap()
}