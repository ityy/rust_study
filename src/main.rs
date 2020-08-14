#![allow(non_snake_case)] //允许驼峰格式
#![allow(unused)] //允许未使用变量

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::rc::Rc;

mod traitT;
mod patternT;
mod into_from_test;
mod lazy_static_test;
mod memory_and_pointer_test;
mod range_test;
mod data_structure;
mod lock_test;


fn main() {
    memory_and_pointer_test::concurrent_write_memory::test();
}


