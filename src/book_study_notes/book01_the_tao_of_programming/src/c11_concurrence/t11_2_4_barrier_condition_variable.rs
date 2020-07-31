//! 屏障（barrier） 与 条件变量（condition variable）

use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;

use failure::_core::time::Duration;

/// 屏障（barrier）
/// 有对齐之意，用于同步线程，当被屏障的线程都执行到此时，再开始执行。
#[test]
fn test_barrier() {
    let mut thread_handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5)); //等待阻塞的线程个数
    for id in 0..5 {
        let barrier_clone = barrier.clone();
        thread_handles.push(thread::spawn(move || {
            println!("thread {} start", id);
            barrier_clone.wait();//当所有线程都执行到此时，再开始执行。
            println!("thread {}  end", id);
        }))
    }

    for x in thread_handles {
        x.join().unwrap();
    }
}

/// 条件变量（condition variable）
/// 有重新让出锁之意，用于获取了锁但条件还不满足，让出锁重新等待。
/// 一个条件变量通常和一个互斥锁搭配使用，用于在满足条件之前，阻塞某个已经获取互斥锁的线程。
#[test]
fn test_condition_variable() {
    let tuple_condition_lock = Arc::new((Mutex::new(0), Condvar::new()));
    let tuple_condition_lock_clone1 = tuple_condition_lock.clone();
    let tuple_condition_lock_clone2 = tuple_condition_lock.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*tuple_condition_lock_clone1;
        let mut x = lock.lock().unwrap();
        println!("thread get lock");
        *x = 1;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*tuple_condition_lock_clone2;
    let mut x = lock.lock().unwrap();
    println!("main thread get lock");
    if *x == 0 {
        println!("x is {}", x);
        // 子线程先获取锁，正常执行
        // 主线程先获取锁，则 cvar.wait()让出锁，并等待唤醒
        x = cvar.wait(x).unwrap();
        println!("x is {}", x);
    } else {
        println!("x is {}", x);
    }
}
