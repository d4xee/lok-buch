use crate::backend::database::lok::Lok;
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

pub fn get_test_lok_1() -> Lok {
    Lok::new_from_raw_data("TEST".to_string(), 114141, "14TE".to_string(), "Roco".to_string(), "ÖBB".to_string(), true, "".to_string())
}

pub fn get_test_lok_2() -> Lok {
    Lok::new_from_raw_data("RRRR".to_string(), 100002, "ABCD".to_string(), "KKLE".to_string(), "DB".to_string(), false, "somewhere".to_string())
}