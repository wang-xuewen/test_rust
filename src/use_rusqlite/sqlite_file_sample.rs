use rusqlite::{Connection, Result};

// #[macro_use]
use crate::log_a;

// #[derive(Debug)]
// pub struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>,
// }

pub fn db_conn() -> Result<Connection> {
    let conn = Connection::open("./sqlite_test.db")?;
    log_a!("get conn ok");
    Ok(conn)
}


pub fn run_sqlite_file_sample() {
    let conn: Connection;
    let result = db_conn();
    match result {
        Ok(cn) => conn = cn,
        Err(err_msg) => {
            log_a!("Error: {}", err_msg);
            return;
        }
    }
}
