//! 自制 Future (三)
//! 在上篇文章中我们学习了如何实现一个高效率的Future(尽量不阻塞, 只有在需要时才会Unpark我们的Task).
//! 今天继续扩展我们的Future: 实现一个Stream Trait.
//! Stream跟Iterators看起来很像: 他们随着时间的推移产生多个相同类型的输出, 与Iterators唯一的区别就是消费的方式不同.
//! 让我们一起尝试使用Reactor来处理Streams吧.
//!
//!
//!
//! ForEach combinator
//! 我们使用一个名为for_each的组合器, 来代替我们手动迭代消费Stream.
//! 查询文档不难发现future::stream实现了ForEach, 所以我们不仅可以迭代, 也可以把stream放入Reactor, 把它作为Future Chain的一部分.
//! 这看起来简直太酷了.现在让我们一步一步来实现一个简单的Stream.
//!
//!
//!
//! impl Stream
//! Stream Trait 与 Future Trait很像:
//! ```
//! pub trait Future {
//!     type Item;
//!     type Error;
//!     fn poll(&mut self) -> Poll<Self::Item, Self::Error>;
//!
//!     // <-- CUT -->
//! }
//!
//! pub trait Stream {
//!     type Item;
//!     type Error;
//!     fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>;
//!
//!     // <-- CUT -->
//! }
//! ```



use std::error::Error;
use std::thread;

use chrono::{DateTime, Utc};
use futures::{Async, Future, Poll, Stream, task};
use futures::future::ok;
use time::Duration;
use tokio_core::reactor::Core;

///让我们一起实现一个简单的stream:
///
/// 新建一个结构体MyStream
struct MyStream {
    current: u32,
    max: u32,
}

/// 给MyStream增加方法
impl MyStream {
    pub fn new(max: u32) -> MyStream {
        MyStream {
            current: 0,
            max: max,
        }
    }
}

/// 给MyStream增加特性
impl Stream for MyStream {
    type Item = u32;
    type Error = Box<dyn Error>;

    //形参传递了一个可变引用, 所以我们可以改变MyStream内部的值
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        //检查MyStream.current是否大于 MyStream.max 如果大于: 返回Ok(Async::Ready(None)), 否则MyStream.current自增1并且返回当前的值.
        match self.current {
            ref mut x if *x < self.max => {
                *x = *x + 1;
                Ok(Async::Ready(Some(*x)))
            }
            //返回None则流结束 和迭代器非常类似
            _ => Ok(Async::Ready(None)),
        }
    }
}


///Consume a stream 消费流
#[test]
fn consume_stream() {
    //新建执行器
    let mut reactor = Core::new().unwrap();
    //新建一个Stream对象
    let my_stream = MyStream::new(5);

    let future = my_stream.for_each(|num| {
        println!("num === {}", num);
        //我们返回的是个Future, 所以我们不仅可以使用Reactor执行future, 也可以跟别的Future, 组合成Future Chain.
        ok(())
    });

    let ret = reactor.run(future);
}
//结果:
//num === 1
//num === 2
//num === 3
//num === 4
//num === 5