//! 测试future的chain必须返回同一类型的Error
use std::{error, fmt};

use futures::Future;
use futures::future::{err, ok};
use tokio_core::reactor::Core;

///自定义两个错误:ErrorA ErrorB
#[derive(Debug, Default)]
pub struct ErrorA {}

///实现显示特性
impl fmt::Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorA!")
    }
}

///实现Error特性
impl error::Error for ErrorA {
    fn description(&self) -> &str {
        "Description for ErrorA"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

// Error B
#[derive(Debug, Default)]
pub struct ErrorB {}

///实现显示特性
impl fmt::Display for ErrorB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorB!")
    }
}

///实现Error特性
impl error::Error for ErrorB {
    fn description(&self) -> &str {
        "Description for ErrorB"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}


fn fut_error_a() -> impl Future<Item=(), Error=ErrorA> {
    err(ErrorA {})
}

fn fut_error_b() -> impl Future<Item=(), Error=ErrorB> {
    err(ErrorB {})
}

pub fn main() {
    let mut reactor = Core::new().unwrap();


    ///分开的方式 没有问题
    let retval = reactor.run(fut_error_a()).unwrap_err();
    println!("fut_error_a == {:?}", retval);
    let retval = reactor.run(fut_error_b()).unwrap_err();
    println!("fut_error_b == {:?}", retval);


    ///合并为chain链式调用 则报错
    /// 后一个chain里返回的错误类型和第一个方法里的不一致. 类型必须统一.
//    let future = fut_error_a().and_then(|_| fut_error_b());


    ///使用map_err()转换错误类型
    let future = fut_error_a()
        .map_err(|e| {
            println!("mapping {:?} into ErrorB", e);
            ErrorB {}
        })
        .and_then(|_| fut_error_b());
    let retval = reactor.run(future).unwrap_err();
    println!("error chain == {:?}", retval);
}