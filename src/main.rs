mod use_rusqlite {
    pub mod sqlite_file_sample;
    pub mod sqlite_mem_sample;
}
// pub mod rust_lang {
//     pub mod borrow_mut;
//     pub mod trait_sample;
// }

use crate::use_rusqlite::{sqlite_file_sample, sqlite_mem_sample};
use test_rust::rust_lang::borrow_mut;
use test_rust::rust_lang::trait_sample;

fn main() {
    borrow_mut::sample_print();
    borrow_mut::rc_refcell::do_rc_refcell();
    sqlite_file_sample::run_sqlite_file_sample();
    sqlite_mem_sample::run_sqlite_mem_sample();
    trait_sample::do_area();
    trait_sample::do_mybox();
}
