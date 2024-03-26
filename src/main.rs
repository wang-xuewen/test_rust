use rusqlite::Connection;

mod use_rusqlite {
    pub mod sqlite_mem_sample;
    pub mod sqlite_file_sample;
}
mod rust_lang {
    pub mod trait_sample;
}

fn main() {
    run_sqlite_file_sample();
    rust_lang::trait_sample::do_area();
}

fn run_sqlite_file_sample() {
    let conn: Connection;
    let result = use_rusqlite::sqlite_file_sample::db_conn();
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

fn run_sqlite_mem_sample() {
    let conn: Connection;

    let result = use_rusqlite::sqlite_mem_sample::db_conn();
    match result {
        Ok(_conn) => {
            conn = _conn;
        }
        Err(err_msg) => {
            println!("Error: {}", err_msg);
            return;
        }
    }

    let _ = use_rusqlite::sqlite_mem_sample::db_create(&conn);
    let _ = use_rusqlite::sqlite_mem_sample::db_insert(&conn);
    let result = use_rusqlite::sqlite_mem_sample::db_select(&conn);
    for person in result {
        println!("db select ok. {:?}", person);
    }
}
