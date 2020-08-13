//! 《Rust 编程之道》 学习记录

//允许出现未使用的导入、未使用的变量、未使用的代码（比如函数）
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod c02_outline;
mod c03_type_system;
mod c04_memory_manage;
mod c05_ownership;
mod c06_function_closure_iterator;
mod c07_structured_programming;
mod c08_string_and_collections;
mod c09_panic_handle;
mod c11_concurrence;

fn main() {
    c11_concurrence::t11_2_1_thread_management::park_unpark();
}