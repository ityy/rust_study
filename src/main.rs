#![allow(non_snake_case)] //允许驼峰格式
#![allow(unused)] //允许未使用变量

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt::Display;

///通过mod引入的模块, 可以再通过use省略路径
use func::test1::two_sum;

///以文件夹为mod的, 文件夹内必须有一个mod.rs存放模块源码
mod func;
mod sliceT;
mod structT;
mod optionT;
mod collections;
mod hashmapT;
mod intoT;
mod traitT;
mod httpT;
mod stringT;
mod patternT;
mod into_from;
mod web_server;
mod http_req;
//mod lazy_static_test;
mod cow_test;
mod pointer_test;


///主方法, 运行其它学习单元的主方法
fn main() {
    pointer_test::mutex_test::test();
}


