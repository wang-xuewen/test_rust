#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use log::{debug, error, info, warn};
use log4rs;
use rust_utils::add;
use std::{default::Default, thread, time::Duration};
use test_rust::log_a;

// mod use_rusqlite {
//     pub mod sqlite_file_sample;
//     pub mod sqlite_mem_sample;
// }

use test_rust::use_rusqlite::{sqlite_file_sample, sqlite_mem_sample};

use test_rust::my_redis_sync;
use test_rust::rust_lang::borrow_mut;
use test_rust::rust_lang::trait_sample;

use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

fn main() {
    log_a!("start ...");
    // -------------------------
    // 创建一个随机数生成器
    let mut rng = OsRng;

    // 生成 RSA 密钥对（2048 位）
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);

    // 要加密的字符串
    let data = "Hello, RSA encryption!";
    println!("Original data: {}", data);

    // 加密
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key
        .encrypt(&mut rng, padding, data.as_bytes())
        .expect("Failed to encrypt data");
    println!("Encrypted data length: {}", encrypted_data.len());

    // 解密
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decrypted_data = private_key
        .decrypt(padding, &encrypted_data)
        .expect("Failed to decrypt data");
    let decrypted_string = String::from_utf8(decrypted_data).expect("Failed to convert to string");
    println!("Decrypted data: {}", decrypted_string);

    // -------------------------
    borrow_mut::sample_print();
    borrow_mut::circular_reference::do_circular_ref();
    borrow_mut::circular_reference_no::do_circular_ref_no();
    borrow_mut::rc_refcell::do_rc_refcell();

    sqlite_file_sample::run_sqlite_file_sample();
    sqlite_mem_sample::run_sqlite_mem_sample();
    trait_sample::do_area();
    trait_sample::do_mybox();

    let add_result = add(1, 2);
    log_a!("1 add 2: {:?}", add_result);

    // thread::spawn(|| {
    //     my_redis_sync::do_my_redis_sync();
    // });

    // test_rust::lib_fn_sample();
    log_a!("main ok....");

    // log4rs sample
    // 加载 log4rs 配置
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // 输出日志
    debug!("This is an debug message.");
    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");

    // loop {
    //     thread::sleep(Duration::from_secs(1));
    //     debug!("This is an debug message.");
    //     info!("This is an info message.");
    //     warn!("This is a warning message.");
    //     error!("This is an error message.");
    // }
}
