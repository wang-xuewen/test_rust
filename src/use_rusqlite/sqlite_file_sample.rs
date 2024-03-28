use rusqlite::{Connection, Result, Statement};

#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn db_conn() -> Result<Connection> {
    let conn = Connection::open("./sqlite_test.db")?;
    println!("get conn ok");
    Ok(conn)
}

pub fn run_sqlite_file_sample() {
    let conn: Connection;
    let result = db_conn();
    match result {
        Ok(_conn) => {
            conn = _conn;
        }
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return;
        }
    }
}