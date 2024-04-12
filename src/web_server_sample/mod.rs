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
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn do_web_server() {
    // 创建一个通道，用于在两个线程之间传递消息
    let (tx, rx) = mpsc::channel();

    let listener = Arc::new(Mutex::new(
        TcpListener::bind("192.168.15.189:7878").unwrap(),
    ));
    let listener_clone = listener.clone();

    let pool = thread_pool::ThreadPool::new(4);

    let mut cnt: i32 = 0;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let r2 = running.clone();

    // 设置 Ctrl+C 信号的处理函数
    ctrlc::set_handler(move || {
        log_a!("Received Ctrl+C, exiting...");
        r.store(false, Ordering::SeqCst);
        tx.send("1").unwrap();
    })
    .expect("Error setting Ctrl+C handler");

    // 在另一个线程中处理连接
    thread::spawn(move || {
        // for stream in listener.incoming().take(5) {
        for stream in listener_clone.lock().unwrap().incoming() {
            if r2.load(Ordering::SeqCst) == false {
                log_a!("listener break ");
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
    });

    let received = rx.recv().unwrap();
    log_a!("mpsc Received: {}", received);

    // 等待 Ctrl+C 信号，然后停止监听
    // while running.load(Ordering::SeqCst) {

    // }

    let _ = call_one();

    log_a!("Sleeping for 1 seconds...");
    thread::sleep(Duration::from_secs(1));
    log_a!("Awake after 1 seconds!");

    log_a!("Shutting down.");
}

fn call_one() -> std::io::Result<()> {
    // 连接到服务器的地址和端口
    let mut stream = TcpStream::connect("192.168.15.189:7878")?;

    // 向服务器发送请求
    stream.write_all(b"Hello from client!")?;

    // 读取服务器的响应
    // let mut buffer = [0; 512];
    // stream.read(&mut buffer)?;

    // 打印服务器的响应
    log_a!("Server response");
    Ok(())
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
