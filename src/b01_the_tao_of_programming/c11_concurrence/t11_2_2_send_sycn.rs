//! Send 和 Sync
//! 从上述Rust的线程管理工具来看，和其它编程语言并无两样。那么它是如何保障线程安全的呢？
//! 这就要归功于std::marker模块中的两个标记特性：Send 和 Sync。
//!
//!