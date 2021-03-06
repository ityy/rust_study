//! # 多线程并发编程
//! Rust为开发者提供的并发编程工具和其它编程语言类似：
//! - 线程管理：在 std::thread 模块中定义了管理线程的各种函数和一些底层同步原语。
//! - 线程同步：在 std::sync 模块中定义了锁，channel，条件变量，屏障等。

use std::{panic, thread};
use std::cell::RefCell;
use std::thread::{Builder, current};
use std::time::Duration;

/// # 创建线程_spawn方式
/// 简单创建线程
#[test]
pub fn create_thread_by_spawn() {
    //存放线程
    let mut v = vec![];
    for id in 0..20 {
        let child_thread = thread::spawn(move || {
            println!("from child: {}", id); //使用move 将id的所有权转让给线程 因为i32实现了copy 相当于复制了一份给线程 不影响外部使用
        });
        v.push(child_thread);
    }
    println!("from main: join before");
    for child in v {
        let join_result = child.join(); //等待子线程结束 字面意思:子线程连接进来
    }
    println!("from main: join after");
}

/// # 创建线程_build方式
/// 可以配置线程属性
#[test]
pub fn create_thread_by_build() {
    let mut v = vec![];
    for id in 0..5 {
        let thread_name = format!("child-{}", id); //用于线程名
        let stack_size: usize = 3 * 1024; //3KB 用于设置线程的栈大小 默认是2MB(Rust升级可能会改) 主线程的栈大小不由Rust决定.
        let builder = Builder::new().name(thread_name).stack_size(stack_size);
        let child = builder.spawn(move || {
            println!("start child: {}", id);
            if id == 3 {
                //捕获恐慌
                let panic_result = panic::catch_unwind(|| {
                    panic!("oh no! i make a panic!"); //产生一个恐慌
                });
                println!("panic child: {} do sm...", current().name().unwrap());
            };
        }).unwrap();
        v.push(child);
    }
    for child in v {
        child.join().unwrap();
    }
}

/// # 线程本地存储（Thread Local Storage，TLS）
/// 线程本地存储是线程独有的存储空间。
#[test]
fn thread_storage() {
    // 使用thread_local!宏便捷生成结构体thread::LocalKey的实例对象。
    // LocalKey是一个结构体，提供了with方法，传入闭包以操作线程本地存储的变量。
    // thread_local!宏经常配合Cell或RefCell以提供内部可变性，因为其生成的是一个结构体。
    thread_local!(static FOO:RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
        assert_eq!(*f.borrow(), 2);
    });


    // 新线程也会生成FOO的实例。
    thread::spawn(|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
            assert_eq!(*f.borrow(), 3);
        })
    });

    // 主线程和新线程的FOO互不影响，互相独立。
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}


/// # 底层同步原语
/// std::thread模块中还提供了一些函数，用来支持底层同步原语，主要包括park/unpark/yield_now函数。
/// - park()      用于阻塞线程，注意不能永久阻塞。也可以通过park_timeout()来指定超时时间。类似传统语言的wait()。
/// - unpark()    用于重启线程。类似传统语言的notify()。
/// - yield_now() 函数用于主动出让时间片。操作系统是抢占式调度线程的，每个线程抢到后会得到一个时间片。如果明知线程什么都不做，为了节省计算时间，可以主动让出抢到的时间片。一般让出本轮时间片后，下一轮还会分到时间片，不用担心唤醒的问题。
// #[test] 使用test测不出线程特性，需使用main方法测试。
pub fn park_unpark() {
    let parked_thread = thread::spawn(|| {
        println!("thread park here");
        thread::park();
        println!("thread unpark!");
    });
    thread::sleep(Duration::from_secs(3));
    println!("main_thread unpark parked_thread");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
}

