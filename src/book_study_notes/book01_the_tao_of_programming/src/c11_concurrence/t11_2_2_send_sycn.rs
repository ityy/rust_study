//! Send 和 Sync
//! 从上述Rust的线程管理工具来看，和其它编程语言并无两样。那么它是如何保障线程安全的呢？
//! 这就要归功于std::marker模块中的两个标记特性：Send 和 Sync。
//!     Send：实现了Send的类型，可以安全的在线程间传递所有权。即跨线程移动，字面意思send：发送，从一个线程发送到另一个线程。
//!     Sync：实现了Sync的类型，可以安全的在线程间传递不可变借用。即跨线程共享，字面意思sync：同步，表示各线程都可以安全的使用不可变借用。
//! 这两个标记，反应了Rust看待线程安全的哲学：多线程共享内存并非线程不安全的问题所在，问题在于错误的共享数据。
//! 查看thread::spawn()源码：
//! ```
//! pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//!     where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
//! {
//!     Builder::new().spawn(f).expect("failed to spawn thread")
//! }
//! ```
//! 其要求传入的闭包F和闭包的返回值T都是被Send标记的。闭包的类型和捕获变量相关，如果捕获变量属于Send，那么闭包就属于Send。
//! 'static表示非引用类型，即Rust禁止引用在线程间传递。如果要在线程间传递引用，必须使用Arc<T>来共享。Arc是Rc的线程安全版本，内部使用了原子类型来计数。
//!
//! Rust 默认为所有类型实现了Send和Sync，为所有原生指针类型实现了!Send和!Sync（即和Send Sync相反，表示不能在线程间安全的传递）
//! 通过这些标记构建的规则，编译器就可以识别线程安全问题，从而在编译期就将问题报出来。
//!
//!
//! Send 帮助编译器检查跨线程错误的示例
//! ```ignore
//! use std::rc::Rc;
//! use std::sync::Arc;
//! use std::thread;
//! #[test]
//! fn test_send_error() {
//!     //! 使用普通类型跨多个线程测试
//!     let mut s = "Hello".to_string();
//!     // 生成3个线程
//!     for _ in 0..3 {
//!         // 发生数据竞争 编译器报错 use of moved value: `s`.  即s已被Send，下一个线程拿不到s的所有权了。
//!         thread::spawn(move || {
//!             s.push_str(" Rust!");
//!         });
//!     }
//!
//!     //! 使用Rc类型跨多个线程测试
//!     let mut s = Rc::new("Hello".to_string());
//!     // 生成3个线程
//!     for _ in 0..3 {
//!         let s_clone=s.clone();
//!         // 编译器报错 cannot be sent between threads safely. 因为Rc实现的是!Send，直接被标记了不能跨线程Send。
//!         thread::spawn(move || {
//!             s_clone.push_str(" Rust!");
//!         });
//!     }
//! }
//! ```
//!
//!
//! 使用Arc类型跨多个线程测试：
//! ```ignore
//! #[test]
//! #[ignore]
//! fn test_send_arc() {
//!     // let mut s = Arc::new("Hello".to_string());
//!     // Arc默认不可变。我们想到了内部可变性Cell和RefCell。但它俩实现了!Send，被禁止在跨线程中使用了。
//!     // 此时需要考虑其它方法了，我们在下一节介绍Mutex<T>锁，来配合Arc使用。
//!     let s = Arc::new("Hello".to_string());
//!     for _ in 0..3 {
//!         let s_clone=s.clone();
//!         thread::spawn(move || {
//!             // 编译器报错 s_clone不可变！
//!             s_clone.push_str(" Rust!");
//!         });
//!     }
//! }
//! ```
