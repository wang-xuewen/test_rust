mod use_rusqlite {
    pub mod sqlite_file_sample;
    pub mod sqlite_mem_sample;
}
pub mod rust_lang {
    pub mod trait_sample;
    pub mod borrow_mut;
}

use crate::use_rusqlite::{sqlite_file_sample,sqlite_mem_sample};
use crate::rust_lang::{trait_sample,borrow_mut};


fn main() {
    sqlite_file_sample::run_sqlite_file_sample();
    sqlite_mem_sample::run_sqlite_mem_sample();
    trait_sample::do_area();
    trait_sample::do_mybox();
    borrow_mut::sample_print();
}
