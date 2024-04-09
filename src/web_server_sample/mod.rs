pub mod thread_pool;
use crate::log_a;

use ctrlc;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn do_web_server() {
    let listener = TcpListener::bind("192.168.15.189:7878").unwrap();

    let pool = thread_pool::ThreadPool::new(4);

    let mut cnt: i32 = 0;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // 设置 Ctrl+C 信号的处理函数
    ctrlc::set_handler(move || {
        log_a!("Received Ctrl+C, exiting...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // for stream in listener.incoming().take(5) {
    for stream in listener.incoming() {
        if !running.load(Ordering::SeqCst) {
            break;
        }
        match stream {
            Ok(stream) => {
                // 在这里处理连接
                cnt += 1;
                log_a!("execute {}", cnt);

                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                log_a!("Error accepting connection: {}", e);
            }
        }
    }

    log_a!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // let current_dir: Result<PathBuf, io::Error>;
    let src_path: PathBuf;

    if let Ok(current_dir) = env::current_dir() {
        src_path = current_dir
            .join("src")
            .join("resource")
            .join("web_server_sample");
    } else {
        log_a!("Failed to get project root directory.");
        return;
    }

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", src_path.join("hello.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", src_path.join("404.html"))
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
