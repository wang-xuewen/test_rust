#![allow(dead_code)] 
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_attributes)]
pub mod rust_lang;
pub mod use_rusqlite;

#[macro_export]
macro_rules! log_a {
    ($($arg:tt)*) => {{
        let current_file = file!();
        let current_line = line!();
        println!("[{}:{}]: {}", current_file, current_line, format!($($arg)*));
    }}
}



