#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use std::usize;
pub mod my_redis_sync;
pub mod rust_lang;
pub mod use_rusqlite;
// use chrono::{DateTime, Local};

#[macro_export]
macro_rules! log_a {
    ($($arg:tt)*) => {{

        // 获取当前本地时间
    let local_time: chrono::DateTime<chrono::Local> = chrono::Local::now();

    // 格式化时间为字符串
    // let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_time = local_time.format(" %m-%d %H:%M:%S").to_string();

        let current_file = file!();
        let current_line = line!();
        let line_head = format!("[{}:{}:{}]: ",current_file, current_line,formatted_time);
        let line_head_fix = $crate::fixed_length_string(&line_head, 45);

        // println!("[{}:{}]: {}", current_file, current_line, format!($($arg)*));
        println!("{}{}", line_head_fix, format!($($arg)*));

    }};
}

pub fn lib_fn_sample() {
    log_a!("lib_fn_sample")
}

pub fn fixed_length_string(input_string: &str, len: usize) -> String {
    let mut result: String = String::with_capacity(len); // 初始化一个容量为 len 的空字符串

    let right_part: &str;
    if input_string.len() > len {
        right_part = &input_string[(input_string.len() - len)..];
    } else {
        right_part = input_string;
    }

    // 将字符串拷贝到结果字符串中
    for c in right_part.chars().take(len) {
        result.push(c);
    }

    // 如果输入字符串长度小于 len，则补充空格字符
    while result.len() < len {
        result.push(' ');
    }

    result
}
