//!自制一个future
//! 单Future轮询
//! 参考:https://studygolang.com/articles/16664
use std::error::Error;

use chrono::{DateTime, Utc};
use futures::{Async, Future, Poll};
use time::Duration;
use tokio_core::reactor::Core;

#[derive(Debug)]
struct WaitForIt {
    message: String,
    until: DateTime<Utc>,
    polls: u64,
}
//message： 自定义字符串消息体
//polls: 轮循次数
//util: 等待时间  这一切都是在单线程内进行的, 这个等待时间是由线程之外的事件比如IO事件等来影响的.
// 所谓轮询只是线程内的loop循环执行到这里进行了判断, 满足则, 不满足则... 所以只消耗1个线程!

impl WaitForIt {
    pub fn new(message: String, delay: Duration) -> WaitForIt {
        WaitForIt {
            message,
            until: Utc::now() + delay,
            polls: 0,
        }
    }
}

impl Future for WaitForIt {
    type Item = String;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let now = Utc::now();
        if self.until < now {
            //等待时间到了
            Ok(Async::Ready(
                format!("{} after {} polls!", self.message, self.polls),
            ))
        } else {
            //等待时间未到
            self.polls += 1;
//            println!("轮询结果:未完成 --> {:?}", self); //在不加这条打印语句时,polls了101k次, 即每秒10万次轮询. 加上打印是20k次.
            futures::task::current().notify();//不加这一句, 则reactor只寻论此future一次.
            Ok(Async::NotReady)
        }
    }
}


//
pub fn main() {
    let mut reactor = Core::new().unwrap();

    let wait1 = WaitForIt::new("hello world!".to_owned(), Duration::seconds(1)); //我们模拟网络请求1秒后才返回
    println!("wait1 == {:#?}", wait1);

    let end = reactor.run(wait1).unwrap();

    println!("执行完毕 == {:#?}", end);
}