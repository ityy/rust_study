use std::panic;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::thread::{Builder, current};

//spawn方式创建线程
pub fn create() {
    //存放线程
    let mut v = vec![];
    for id in 0..5 {
        let child_thread = thread::spawn(move || {
            println!("from child: {}", id); //使用move 将id的所有权转让给线程 因为i32实现了copy 相当于复制了一份给线程 不影响外部使用
        });
        v.push(child_thread);
    }
    println!("from main: join before");
    for child in v {
        child.join(); //等待子线程结束 字面意思:子线程连接进来
    }
    println!("from main: join after");
}

//build方式更详细的创建线程
pub fn build() {
    let mut v = vec![];
    for id in 0..5 {
        let thread_name = format!("child-{}", id); //用于线程名
        let size: usize = 3 * 1024; //3KB 用于设置线程的栈大小 默认是2MB(Rust升级可能会改) 主线程的栈大小不由Rust决定.
        let builder = Builder::new().name(thread_name).stack_size(size);
        let child = builder.spawn(move || {
            println!("in child: {}", id);
            if id == 3 {
                //捕获恐慌
                panic::catch_unwind(|| {
                    panic!("oh no!"); //产生一个恐慌
                });
                println!("in {} do sm", current().name().unwrap());
            };
        }).unwrap();
        v.push(child);
    }
    for child in v {
        child.join().unwrap();
    }
}