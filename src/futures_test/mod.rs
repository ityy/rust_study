///future意味将来
/// 表示写好的代码,方法串联好,待到使用时才运行.
use std::error::Error;

use futures::Future;
use futures::future::ok;
use tokio_core::reactor::Core;

use error::{ErrorA, ErrorB};

pub mod error;
pub mod future_031_single;
pub mod future_032_multi;
pub mod future_041_wait;
pub mod future_042_thread;

//原始function
fn my_fn() -> Result<u32, Box<Error>> {
    Ok(100)
}

//future
fn my_fut() -> impl Future<Item=u32, Error=Box<Error + 'static>> {
    ok(100)
}

pub fn main() {
    //原始调用
    let retval = my_fn().unwrap();
    println!("{:?}", retval);

    //调用future
    let mut reactor = Core::new().unwrap();
    let retval = reactor.run(my_fut()).unwrap();
    println!("{:?}", retval);
}