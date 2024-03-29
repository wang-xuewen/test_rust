use rusqlite::{Connection, Result};

#[derive(Debug)]
#[allow(dead_code)]
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

#[allow(unused_assignments)]
#[allow(unused_variables)]
pub fn run_sqlite_file_sample() {
    let conn: Connection;
    let result = db_conn();
    match result {
        Ok(cn) => conn = cn,
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return;
        }
    }
}
