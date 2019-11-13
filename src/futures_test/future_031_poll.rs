//! 自制 Future (一)
//! 单Future轮询 与 多Future轮询
//! 参考:https://studygolang.com/articles/16664
use std::error::Error;

use chrono::{DateTime, Utc};
use futures::{Async, Future, Poll};
use time::Duration;
use tokio_core::reactor::Core;

///定义一个结构体, 并为其实现Future特性
#[derive(Debug)]
struct WaitForIt {
    //message： 自定义字符串消息体
    message: String,
    //util: 等待时间  这一切都是在单线程内进行的, 这个等待时间是由Future单线程模型之外的事件比如IO事件等来影响的.
    until: DateTime<Utc>,
    //polls: 轮循次数
    polls: u64,
}

// 所谓轮询只是线程内的loop循环执行到这里进行了判断, 满足则, 不满足则... 所以只消耗1个线程!
impl WaitForIt {
    ///新建WaitForIt
    pub fn new(message: String, delay: Duration) -> WaitForIt {
        WaitForIt {
            message,
            until: Utc::now() + delay,
            polls: 0,
        }
    }
}

/// 实现Future特性
impl Future for WaitForIt {
    //定义返回类型Item
    type Item = String;
    //定义返回类型Error
    type Error = Box<Error>;

    ///实现poll方法
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let now = Utc::now();
        if self.until < now {
            //等待时间到了, 返回Ready, 并携带处理结果
            Ok(Async::Ready(
                format!("{} after {} polls!", self.message, self.polls),
            ))
        } else {
            //等待时间未到, 被轮询次数+1, 返回NotReady, 什么都不携带
            self.polls += 1;
//            println!("轮询结果:未完成 --> {:?}", self); //在不加这条打印语句时,polls了101k次, 即每秒10万次轮询. 加上打印是20k次.
            futures::task::current().notify();//unpark此Future. 不加这一句, 则询问未准备好后此Future会被park, 不会再被轮询到.
            Ok(Async::NotReady)
        }
    }
}


///单Future轮询
#[test]
pub fn single_test() {
    //获取执行器
    let mut reactor = Core::new().unwrap();

    // 新建一个Future对象
    // 我们模拟网络请求1秒后才返回
    let wait1 = WaitForIt::new("hello world!".to_owned(), Duration::seconds(1));
    println!("wait1 == {:#?}", wait1);
    let end = reactor.run(wait1).unwrap();
    println!("执行完毕 == {:#?}", end);
}

//打印结果:
//执行完毕 == "hello world! after 96643 polls!"


//结果分析:
//在loop中被询问了96643次是否准备好....
//目前为止这和自己写一个方法不断判断时间是否到了, 到了则向下执行, 不到则继续循环判断没有很大的区别
//我们继续向下扩展学习


/// 多Future轮询
#[test]
pub fn multi_test() {
    let mut reactor = Core::new().unwrap();

    //创建多个Future特性的对象
    let wait1 = WaitForIt::new("我是任务1".to_owned(), Duration::seconds(1)); //我们模拟网络请求1秒后才返回
    println!("wait1 == {:#?}", wait1);
    let wait2 = WaitForIt::new("我是任务2".to_owned(), Duration::seconds(2)); //我们模拟网络请求1秒后才返回
    println!("wait2 == {:#?}", wait1);

    //组合起来
    let v = vec![wait1, wait2];
    //方式1 join_all 全部完成时reactor返回 这里的关键点是两个请求是交错的：第一个Future被调用，然后是第二个，然后是第一个，依此类推，直到两个完成。
//    let sel = futures::future::join_all(v);
    //方式2 select_all 返回第一个完成的  这对于实现超时很有用。
    let sel = futures::future::select_all(v);
    let end = reactor.run(sel).unwrap();
    println!("执行完毕 == {:#?}", end);
}
//打印结果:
//方式1: 结果是一个数组
//执行完毕 == [
//    "我是任务1 after 10281 polls!",
//    "我是任务2 after 29980 polls!",
//]

//方式2: 结果是一个元组
//执行完毕 == (
//    "我是任务1 after 9959 polls!",
//    0,
//    [
//        WaitForIt {
//            message: "我是任务2",
//            until: 2019-11-13T03:49:36.872062400Z,
//            polls: 9959,
//        },
//    ],
//)


//到选在为止的方式仍然可以用循环判断来实现, 我们继续向下学习

