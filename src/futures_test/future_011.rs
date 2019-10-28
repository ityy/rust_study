//!future意味将来
//! 表示写好的代码,方法串联好,待到使用时才运行.
use std::fmt::Error;

use futures::Future;
use futures::future::ok;
use tokio_core::reactor::Core;

use crate::futures_test::error::{ErrorA, ErrorB};

//原始function
fn my_fn() -> Result<u32, Box<Error>> {
    Ok(100)
}

//future
fn my_fut() -> impl Future<Item=u32, Error=Box<Error>> {
    ok(100)
}

#[test]
pub fn test1() {
    //原始调用
    let retval = my_fn().unwrap();
    println!("{:?}", retval);

    //调用future
    let mut reactor = Core::new().unwrap();
    let retval = reactor.run(my_fut()).unwrap();
    println!("{:?}", retval);
}

fn my_fn_squared(i: u32) -> Result<u32, Box<Error>> {
    Ok(i * i)
}

fn my_fut_squared(i: u32) -> impl Future<Item=u32, Error=Box<Error>> {
    ok(i * i)
}

#[test]
pub fn test2() {
    //原始调用
    let retval = my_fn().unwrap();
    println!("原始调用 {:?}", retval);
    let retval2 = my_fn_squared(retval).unwrap();
    println!("原始调用 {:?}", retval2);

    //调用future
    let mut reactor = Core::new().unwrap();
    let retval = reactor.run(my_fut()).unwrap();
    println!("调用future {:?}", retval);
    let retval2 = reactor.run(my_fut_squared(retval)).unwrap();
    println!("调用future {:?}", retval2);

    //调用future组合
    let chained_future = my_fut().and_then(|retval| my_fut_squared(retval));
    let retval2 = reactor.run(chained_future).unwrap();
    println!("调用future组合 {:?}", retval2);
}