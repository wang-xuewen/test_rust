// 自建简易版redis同步服务器例子
pub mod commands;

use crate::log_a;
use commands::process_client_request;
use lazy_static::lazy_static;
use resp::Decoder;
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Write};
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

type STORE = Mutex<HashMap<String, String>>;

lazy_static! {
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}

pub fn do_my_redis_sync() {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6378".to_owned());

    // let listener = Arc::new(Mutex::new(TcpListener::bind(&addr).unwrap()));
    // let listener_clone = listener.clone();
    let listener =TcpListener::bind(&addr).unwrap();

    log_a!("rudis_sync listening on {} ...", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        log_a!("New connection from: {:?}", stream);
        handle_client(stream);
        log_a!("New connection done");

        // thread::sleep(Duration::from_millis(3000));
    }
}

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    log_a!("========1=");
    let decoder = Decoder::new(&mut stream).decode();
    log_a!("========2=");
    match decoder {
        Ok(v) => {
            let reply = process_client_request(v);
            log_a!("========3=");
            if !reply.is_empty() {
                log_a!("handle_client start");
                stream.get_mut().write_all(&reply).unwrap();
                log_a!("handle_client end");
            }else {
                log_a!("handle_client reply is empty");
            }
        }
        Err(e) => {
            println!("Invalid command: {:?}", e);
            let _ = stream.get_mut().shutdown(Shutdown::Both);
        }
    };
}
