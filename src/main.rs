use rusqlite::Connection;

mod use_rusqlite {
    pub mod sqlite_file_sample;
    pub mod sqlite_mem_sample;
}
mod rust_lang {
    pub mod trait_sample;
}

fn main() {
    use_rusqlite::sqlite_file_sample::run_sqlite_file_sample();
    use_rusqlite::sqlite_mem_sample::run_sqlite_mem_sample();
    rust_lang::trait_sample::do_area();
    rust_lang::trait_sample::do_mybox();
}


