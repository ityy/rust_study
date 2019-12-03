//! 自制 Future (二)
//! 更贴近日常使用场景的Future
//! 测试 旋转等待与休眠等待
//!
//! 参考:https://studygolang.com/articles/16744
//! 之前三篇文章我们阐述如何处理Future的基础知识， 我们现在能组织多个Future成为一个Future chain, 执行他们,甚至创建他们.但是到现在我们的Future还没有贴近我们日常的使用场景。(But, so far, our futures are not really delegating the execution to another thing.)
//! 在Part-3中我们用了粗暴的方法来Unpark Future。虽然解决了问题，并且使Reactor变得相对高效，但是这不是最佳实践。今天就让我们换种更好的方式去实现Future。

use std::error::Error;
use std::thread;

use chrono::{DateTime, Utc};
use futures::{Async, Future, Poll, task};
use time::Duration;
use tokio_core::reactor::Core;

/// 创建一个最简单的Timer Future
/// 这一次，我们不会立即Unpark Future Task, 而是一直Parked， 直到这个Future准备完为止.
/// 我们该怎样实现？ 最简单的方式就是再委派一个线程。这个线程将等待一段时间， 然后Unpark我们的Future Task. 用这个线程模拟IO完毕后的回调.
pub struct WaitInAnotherThread {
    //结束时间
    end_time: DateTime<Utc>,
    //是否已在运行
    running: bool,
}

impl WaitInAnotherThread {
    pub fn new(how_long: Duration) -> WaitInAnotherThread {
        WaitInAnotherThread {
            end_time: Utc::now() + how_long,
            running: false,
        }
    }

    //Spin wait 旋转等待 缺点:CPU占用高,浪费CPU周期
    pub fn wait_spin(&self) {
        //利用while执行无意义的循环, 直到到时为止
        while Utc::now() < self.end_time {
            //什么都不做
        }
        //准备好了
        println!("时机已到 == {:?}!", self.end_time);
    }

    //Sleep wait 休眠等待
    pub fn wait_blocking(&self) {
        //给while增加一个循环体, 代码是sleep一个时间差, 正好醒的时候时间到.
        while Utc::now() < self.end_time {
            let delta_sec = self.end_time.timestamp() - Utc::now().timestamp();
            if delta_sec > 0 {
                thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
            }
        }
        println!("时机已到 == {:?}!", self.end_time);
    }

    ///供被轮询时,没有准备好的情况下创建监听线程的函数
    fn run(&mut self, task: task::Task) {
        let lend = self.end_time;

        thread::spawn(move || {
            println!("旁线程:开始运行");

            while Utc::now() < lend {
                let delta_sec = lend.timestamp() - Utc::now().timestamp();
                if delta_sec > 0 {
                    println!("旁线程:开始休眠等待");
                    thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
                    println!("旁线程:结束休眠");
                }
                println!("旁线程:upark Future");
                //unpark这个Future task
                task.notify();
            }
            println!("旁线程:结束 {:?}!", lend);
        });
    }
}

///原始调用方式
#[test]
pub fn primitive_test() {
    //新建
    let wiat = WaitInAnotherThread::new(Duration::seconds(30));
    println!("started");
    wiat.wait_blocking();
    println!("completed");
}


///Future
///我们还没有实现Future Trait, 所以，我们现在实现它。
//impl Future for WaitInAnotherThread {
//    type Item = ();
//    type Error = Box<Error>;
//
//    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
//        //此处在poll内sleep, 等于把Reactor给sleep了, 这样做就完蛋了.
//        while Utc::now() < self.end_time {
//            let delta_sec = self.end_time.timestamp() - Utc::now().timestamp();
//            if delta_sec > 0 {
//                thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
//            }
//        };
//        println!("准备好了 {:?}", self.end_time);
//        Ok(Async::Ready(()))
//    }
//}

/// 一个Reactor的最佳实践应该至少包含下面几条：
/// 当主Task需要等待别的Task时，应该停止它。
/// 不要阻塞当前线程。
/// 任务完成时向Reactor发送信号。
/// 我们要做的是创建另一个睡眠线程. 睡眠的线程是不会占用CPU资源。所以Reactor还会高效的工作着。当这个Sleep Thread醒来后, 它会Unpark这个任务, 并且通知Reactor。
impl Future for WaitInAnotherThread {
    type Item = ();
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("轮询WaitInAnotherThread:开始");
        if Utc::now() < self.end_time {
            println!("轮询WaitInAnotherThread:task 未准备好");
            // 1 当没有准备好时 创建一个并行线程
            // 2 需要一个字段来判断是否已创建了并行线程
            // 需要注意的是当Future被轮询之前，这些代码是不会被执行的, 也没必要创建一个并行线程.
            // 当然我们还会检测当前时间是否大于过期时间，如果大于，也没必要产生另外一个线程。
            // 如果end_time大于当前的时间并且另一个线程没有被创建，程序就会立即创建一个新的线程。
            // 然后程序会返回Ok(Async::NotReady()), 与我们Part-3中所做的相反，我们不会在这里Unpark Task.
            // 这是另一个线程应该做的事情。在别的实现中，例如IO，唤醒我们的主线程的应该是操作系统。
            if !self.running {
                println!("轮询WaitInAnotherThread:旁线程没有运行, 开始运行!");
                self.run(task::current());
                self.running = true;
            }
            println!("轮询WaitInAnotherThread:park Future");
            //这里有两件事情需要注意下.
            //我们将Task引用传递给，另一个并行的线程。这很重要，因为我们不能在单独的线程里使用task::current.
            //我们不能将self移动到闭包中，所以我们需要转移所有权至lend变量.为啥这样做？
            //Rust中的线程需要实现具有'static生命周期的Send Trait。
            //task自身实现了上述的要求所以可以引用传递。但是我们的结构体没有实现，所以这也是为什么要移动end_time的所有权。这就意味着当线程被创建后你不能更改end_time.

            Ok(Async::NotReady)
        } else {
            println!("轮询WaitInAnotherThread:task 准备好了");
            Ok(Async::Ready(()))
        }
    }
}

///测试一下代有Future特性的WaitInAnotherThread对象
#[test]
fn future_test() {
    //新建执行器
    let mut reactor = Core::new().unwrap();
    //新建Future
    let wiat = WaitInAnotherThread::new(Duration::seconds(10));
    println!("执行器:开始运行...");
    let ret = reactor.run::<WaitInAnotherThread>(wiat).unwrap();
    println!("执行器:运行完成.\n{:?}", ret);
}

//打印结果:
//执行器:开始运行...
//轮询WaitInAnotherThread:开始
//轮询WaitInAnotherThread:task未准备好
//轮询WaitInAnotherThread:旁线程没有运行, 开始运行!
//轮询WaitInAnotherThread:park Future
//旁线程:开始运行
//旁线程:开始休眠等待
//旁线程:结束休眠
//旁线程:upark Future
//旁线程:结束 2019-11-13T07:14:19.923079200Z!
//轮询WaitInAnotherThread:开始
//轮询WaitInAnotherThread:task准备好了
//执行器:运行完成.
//()

//结果分析:
// 还是单执行器 单Future, 尚未体现异步优点
//到目前未知我们完整的实现了没有阻塞的real life future. 所以也没有浪费CPU资源。除了这个例子你还能想到与此相同的应用场景吗？
//尽管RUST早都有现成的Crate帮我们实现好了。但是了解其中的工作原理还是对我们有很大的帮助。