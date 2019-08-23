//! 测试 旋转等待与休眠等待
//!
//! 参考:https://studygolang.com/articles/16744
//! 之前三篇文章我们阐述如何处理Future的基础知识， 我们现在能组织多个Future成为一个Future chain, 执行他们,甚至创建他们.但是到现在我们的Future还没有贴近我们日常的使用场景。(But, so far, our futures are not really delegating the execution to another thing.)
//! 在Part-3中我们用了粗暴的方法来Unpark Future。虽然解决了问题，并且使Reactor变得相对高效，但是这不是最佳实践。今天就让我们换种更好的方式去实现Future。

use std::thread;

use chrono::{DateTime, Utc};
use time::Duration;

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
}


pub fn main() {
    let wiat = WaitInAnotherThread::new(Duration::seconds(30));
    println!("started");
    wiat.wait_blocking();
    println!("completed");
}