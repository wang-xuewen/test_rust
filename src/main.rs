#![allow(dead_code)] 
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_attributes)]


// mod use_rusqlite {
//     pub mod sqlite_file_sample;
//     pub mod sqlite_mem_sample;
// }

// use crate::use_rusqlite::{sqlite_file_sample, sqlite_mem_sample};
use test_rust::use_rusqlite::{sqlite_file_sample, sqlite_mem_sample};

use test_rust::rust_lang::borrow_mut;
use test_rust::rust_lang::trait_sample;

fn main() {
    borrow_mut::sample_print();
    borrow_mut::circular_reference::do_circular_ref();
    borrow_mut::rc_refcell::do_rc_refcell();

    sqlite_file_sample::run_sqlite_file_sample();
    sqlite_mem_sample::run_sqlite_mem_sample();
    trait_sample::do_area();
    trait_sample::do_mybox();
}
