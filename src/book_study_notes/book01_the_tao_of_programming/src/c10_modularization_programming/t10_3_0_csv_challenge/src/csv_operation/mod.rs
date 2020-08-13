//! # 读写文件操作模块
//! 文件夹型模块使用 mod.rs 代表模块本身
//!
//! 文件夹内还可以增加子模块，则本模块为父模块

use std::{
    fs::File,
    io::{Read, Write},
    //  Rust有两种路径抽象：Path和PathBuf，有点类似&str String的关系。且PathBuf屏蔽了操作系统的差异。
    path::PathBuf,
};

use crate::err::Error;

// 子模块
pub mod read;
pub mod write;

