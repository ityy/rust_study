//! 使用channel来进行线程间通信,从而实现无锁并发。
//! 一句老话：不要通过共享内存来通信，而应该使用通信来共享内存。
//! 基于消息通信的并发模型有两种：
//!     Actor   代表语言Erlang
//!             主角为Actor，主角为Actor之间直接发送，接收消息。耦合度高。
//!             Actor之间直接通信。
//!     CSP     代表语言Golang
//!             主角为Channel，不关注谁发送消息，谁接收消息。耦合度低。
//!             依靠Channel通信。
//! Rust标准库选择了CSP并发通信模型，std::sync::mpsc模块提供了Channel机制。
//!     mpsc意为多发单收，p是生产，c是消费。且是FIFO的先进先出队列。
//!     mpsc限制了发送者可以克隆，而接收者禁止克隆。
//!
//! 生产者-消费者模式与Channel
//!     生产者-消费者模式指通过一个中间层来解决生产者-消费者的耦合问题，生产者-消费者不直接通信，而是分别与中间层通信。这样就消解了生产者与消费者的差异。
//!
//! 三种CSP进程：
//!     Sender      发送异步消息
//!     SyncSender  发送同步消息
//!     Receiver    接收消息
//! 两种Channel类型：
//!     异步无界Channel
//!         对应channel()函数，返回(Sender,Receiver)元组
//!         异步指发送消息不会阻塞，无界指理论上缓冲区无限大。
//!     同步有界Channel
//!         对应sync_channel()函数，返回(SyncSender,Receiver)元组
//!         可以指定缓冲区大小，缓冲区满时，发送消息会导致阻塞，直到缓冲区可用。
//!         当缓冲区大小为0时，此时发送和接收消息是一个原子操作。
//! 错误处理：
//!     channel发送和接收消息都会返回Result用于错误处理。通常直接使用unwrap在线程间传播错误，及早发现问题。

use std::sync::mpsc::{channel, sync_channel};
use std::thread;

/// 单发单收示例
/// 这种只有一个发送者和接收者的情况，也叫做流通道（streaming channel），rust会优化为spsc来提高性能。
#[test]
fn test_send_recv() {
    // 可以指定消息类型，也可以由编译器推断。
    let (tx, rx) = channel::<i32>();
    // 另一个线程发送
    thread::spawn(move || {
        tx.send(10).unwrap();
    });

    // 主线程接收
    let message = rx.recv().unwrap();
    println!("message is {}", message);
}

/// 多发单收示例
/// 这种有多个发送者和一个接收者的情况，也叫做共享通道（sharing channel）。
#[test]
fn test_mpsc() {
    let (tx, rx) = channel::<i32>();
    // 同时开启10个发送
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("thread {} start", i);
            tx.send(i).unwrap(); //异步通道不会阻塞，发送完直接执行下一句，不关心接受者是否已接收。
            println!("thread {} end", i);
        });
    }

    for i in 0..10 {
        let message = rx.recv().unwrap();
        println!("message is {}", message);
    }
}

/// 同步通道示例
#[test]
fn test_sync_mpsc() {
    let (tx, rx) = sync_channel::<i32>(1); // 设置缓冲区大小为1
    tx.send(0).unwrap();
    thread::spawn(move || {
        println!("thread {} start", 1);
        tx.send(1).unwrap(); //同步通道缓冲区满之后会阻塞，直到缓冲区可用，才会继续向下执行。
        println!("thread {} end", 1);
    });

    for i in 0..2 {
        let message = rx.recv().unwrap();
        println!("message is {}", message);
    }
}

/// channel迭代与死锁示例
#[test]
fn test_iter() {
    let (tx, rx) = channel::<i32>();
    // 同时开启10个发送
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("thread {} start", i);
            tx.send(i).unwrap(); //异步通道不会阻塞，发送完直接执行下一句，不关心接受者是否已接收。
            println!("thread {} end", i);
        });
    }

    //drop(tx); //迭代之前，手动消亡掉主线程的tx。否则会发送死锁，因为主线程不再发送信息却仍持有tx，导致rx无限迭代。

    // 通过迭代的方式获取消息。
    // 注意，tx和rx是依存的，只要tx还存在，rx就会阻塞一直迭代。tx消亡，rx就不再存在next的信息，从来结束迭代。
    for i in rx.iter() {
        println!("message is {}", i);
    }
}