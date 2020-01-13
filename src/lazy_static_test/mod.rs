//! 静态全局变量实现
//! lazy_static = "1.3.0"
//!
//! Rust 静态变量
//! ```
//! //rust不支持高级类型的静态全局变量
//! //rust的静态全局变量都是不安全的，需要在unsafe内使用
//! static mut NAME: &str = "hello";
//! ```
//!
//! 使用lazy_static实现高级类型的静态全局变量
//!
use std::collections::HashMap;
use std::sync::Mutex;

pub mod lazy_static_hashmap;

//lazy_static! {
//    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
//}
//库提供的宏
lazy_static! {
    //创建一个静态引用, 使用Mutex互斥器为其加锁
    static ref ARRAY: Mutex<HashMap<u32, String>> = Mutex::new(HashMap::new());
}

fn do_a_call() {
    ARRAY.lock().unwrap().insert(1, "a".to_string());
}


pub fn main() {
    do_a_call();
//    do_a_call();
//    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().get(&1).unwrap());
}