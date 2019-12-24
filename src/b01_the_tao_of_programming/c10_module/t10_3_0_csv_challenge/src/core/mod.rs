pub mod read;
pub mod write;
use crate::err::Error;
use std::{
  //  Rust有两种路径抽象：Path和PathBuf，有点类似&str String的关系。且PathBuf屏蔽了操作系统的差异。
  path::PathBuf,
  fs::File,
  io::{Read, Write},
};
