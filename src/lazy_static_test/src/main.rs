//! # 静态全局变量实现
//! Rust 静态变量
//! ```
//! //rust不支持高级类型的静态全局变量
//! //rust的静态全局变量都是不安全的，需要在unsafe内使用
//! static mut NAME: &str = "hello";
//! ```
//!
//! 使用lazy_static可以实现高级类型的静态全局变量
//! ```
//! #[macro_use]
//! extern crate lazy_static;
//!
//! use std::collections::HashMap;
//!
//! lazy_static! {
//!     static ref HASHMAP: HashMap<u32, &'static str> = {
//!         let mut m = HashMap::new();
//!         m.insert(0, "foo");
//!         m.insert(1, "bar");
//!         m.insert(2, "baz");
//!         m
//!     };
//!     static ref COUNT: usize = HASHMAP.len();
//!     static ref NUMBER: u32 = times_two(21);
//! }
//!
//! fn times_two(n: u32) -> u32 { n * 2 }
//!
//! fn main() {
//!     println!("The map has {} entries.", *COUNT);
//!     println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
//!     println!("A expensive calculation on a static results in: {}.", *NUMBER);
//! }
//! ```

#[macro_use]
extern crate lazy_static;


use std::collections::HashMap;
use std::sync::Mutex;

//全局静态变量宏, 支持vec, hashmap
lazy_static! {
    //带互斥锁的map,可以获取锁后进行读写操作
    static ref STATIC_MAP_WITH_LOCK: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    //这个库声明的都是静态不可变引用, 不加锁的情况下下方的map只能读不能写, 可以在初始化时添加数据
    //但可以使用c指针(即原始指针)的形式破除不可写的限制
    static ref STATIC_MAP: HashMap<String, String> =HashMap::new();

    //创建一个独立锁. 由于锁是在作用域结束后自动解锁, 可以单独作为锁使用.
    static ref STATIC_LOCK: Mutex<i32> = Mutex::new(0);
}

/// # 测试读全局map
fn map_read() {
    // 获取STATIC_MAP的数字地址
    let static_map_intaddr: usize = &*STATIC_MAP as *const HashMap<String, String> as usize;
    // 数字地址转原生指针
    let pointer: *mut HashMap<String, String> = static_map_intaddr as *mut HashMap<String, String>;
    unsafe {
        // 原生指针转Rust引用
        let map = &mut *pointer;
        let k = map.get("a").unwrap();
        let s = map.get("b").unwrap();
        println!("{} {}", k, s);
    }
}

/// # 测试写全局map
fn map_write() {
    // 获取STATIC_MAP的数字地址
    let static_map_intaddr: usize = &*STATIC_MAP as *const HashMap<String, String> as usize;
    // 数字地址转原生指针
    let pointer: *mut HashMap<String, String> = static_map_intaddr as *mut HashMap<String, String>;
    unsafe {
        // 原生指针转Rust引用
        let map = &mut *pointer;
        map.insert("a".to_string(), "aaa".to_string());
        map.insert("b".to_string(), "bbb".to_string());
    }
}

fn main() {
    map_write();
    map_read();
}
