//! 线程唤醒,unpark任务并通知Reactor
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

pub struct WaitInAnotherThread {
    end_time: DateTime<Utc>,
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
        while Utc::now() < self.end_time {}
        println!("时机已到 == {:?}!", self.end_time);
    }

    //Sleep wait 休眠等待
    pub fn wait_blocking(&self) {
        while Utc::now() < self.end_time {
            let delta_sec = self.end_time.timestamp() - Utc::now().timestamp();
            if delta_sec > 0 {
                thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
            }
        }
        println!("时机已到 == {:?}!", self.end_time);
    }

    //创建一个新线程
    fn run(&mut self, task: task::Task) {

        //我们不能将self移动到闭包中，所以我们需要转移所有权至lend变量.
        //Rust中的线程需要实现具有'static生命周期的Send Trait。
        //task自身实现了上述的要求所以可以引用传递。但是我们的结构体没有实现，所以这也是为什么要移动end_time的所有权。这就意味着当线程被创建后你不能更改end_time.
        let lend = self.end_time;

        thread::spawn(move || {
            while Utc::now() < lend {
                let delta_sec = lend.timestamp() - Utc::now().timestamp();
                if delta_sec > 0 {
                    //让监测线程休眠. 到时间后才告诉Reactor
                    thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
                }
                //unpark 任务
                task.notify();
            }
            println!("side thread done! time is {:?}!", lend);
        });
    }
}

impl Future for WaitInAnotherThread {
    type Item = ();
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if Utc::now() < self.end_time {
            println!("not ready, task parking!");

            if !self.running {
                println!("side thread not running! starting now!");
                //当Future被轮询之前，这些代码是不会被执行的。当然我们还会检测当前时间是否大于过期时间，如果大于，也不会产生另外一个线程。
                self.run(task::current());
                self.running = true;
            }

            Ok(Async::NotReady)
        } else {
            println!("已经准备好!");
            Ok(Async::Ready(()))
        }
    }
}

pub fn main() {
    let mut reactor = Core::new().unwrap();
    let wiat = WaitInAnotherThread::new(Duration::seconds(30));
    println!("started");
    let ret = reactor.run(wiat).unwrap();
    println!("completed");
    println!("ret == {:?}", ret);
}