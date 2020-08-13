//! # 定义统一的错误类型
//! 使各种其它错误 可以转换为此统一错误
//!
//! 本策略不同于Error trait，属于静态转换，性能高。

use std::io;

/// # 定义统一错误枚举，将不同的错误都作为枚举值。
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Program(&'static str),
}

/// 实现其它错误转本错误的方法from()
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Error {
        Error::Program(e)
    }
}