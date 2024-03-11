

use rusqlite::Connection;


mod test_rusqlite {
    pub mod sqlite_mem_sample;
}

fn main() {
    let conn: Connection;

    let result = test_rusqlite::sqlite_mem_sample::db_conn();
    match result {
        Ok(number) => {
            conn = number;
        }
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return;
        }
    }

    let _ = test_rusqlite::sqlite_mem_sample::db_create(&conn);
    let _ = test_rusqlite::sqlite_mem_sample::db_insert(&conn);
    let result = test_rusqlite::sqlite_mem_sample::db_select(&conn);
    for person in result {
        println!("db select ok. {:?}", person);
    }
}
