#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_attributes)]

use test_rust::log_a;

// mod use_rusqlite {
//     pub mod sqlite_file_sample;
//     pub mod sqlite_mem_sample;
// }

use test_rust::use_rusqlite::{sqlite_file_sample, sqlite_mem_sample};

use test_rust::rust_lang::borrow_mut;
use test_rust::rust_lang::trait_sample;
use test_rust::web_server_sample;
use test_rust::my_redis_sync;
use std::thread;

fn main() {
    log_a!("start ...");

    borrow_mut::sample_print();
    borrow_mut::circular_reference::do_circular_ref();
    borrow_mut::circular_reference_no::do_circular_ref_no();
    borrow_mut::rc_refcell::do_rc_refcell();

    sqlite_file_sample::run_sqlite_file_sample();
    sqlite_mem_sample::run_sqlite_mem_sample();
    trait_sample::do_area();
    trait_sample::do_mybox();

    thread::spawn(|| {
        my_redis_sync::do_my_redis_sync();
    });
    web_server_sample::do_web_server();
   

    // test_rust::lib_fn_sample();
    log_a!("main ok....");
}
