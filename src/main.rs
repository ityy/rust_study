#![allow(non_snake_case)] //允许驼峰格式
#![allow(unused)] //允许未使用变量

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;

///以文件夹为mod的, 文件夹内必须有一个mod.rs存放模块源码
mod sliceT;
mod optionT;
mod collections;
mod hashmapT;
mod intoT;
mod traitT;
mod stringT;
mod patternT;
mod into_from;
mod lazy_static_test;
mod b04_memory_and_pointer;
mod range_test;
mod data_structure;
mod lock_test;
mod b01_the_tao_of_programming;


///主方法, 运行其它学习单元的主方法
fn main() {
    hashmapT::main();
}


