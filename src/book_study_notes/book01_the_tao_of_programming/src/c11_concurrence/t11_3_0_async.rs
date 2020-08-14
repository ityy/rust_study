//! # 异步并发
//! t11_1_0_general_concepts 已经介绍相关概念。
//!
//! 异步并发的三个阶段：
//! - 第一阶段
//!    - 使用回调函数实现
//!    - 产生回调地狱问题
//! - 第二阶段
//!    - 使用Promise/Future并发模型，解决回调地狱问题
//!    - 代码冗余巨大
//! - 第三阶段
//!    - async/await解决方案，号称异步的“终极解决方案”
//!
//! 大多编程语言都支持异步并发，但支持到第三阶段的并不多。比如异步开发大放异彩的JS，也是到ES7版本才支持。
//! Rust在1.0发布时并不支持异步，主要解决的是并发安全问题。
//! 随着版本迭代，Rust确定了新的发展路线：成为能开发高性能网络服务的首选语言。
//! 有了这个路线指引，Rust对异步的支持将会朝着完美前进。
//!
//! Rust异步分三个阶段：
//! - 生成器阶段
//! - Future模型阶段
//! - async/await语法阶段
//!
//! 要支持async/await异步开发，最好能有协程支持。所以Rust的第一步是引入协程（Coroutine）。
//!
//! 协程的实现分两种：
//! - 有栈协程
//!    - 每个协程带独立的栈
//!    - 功能强，性能弱
//! - 无栈协程
//!    - 基于状态机，具体应用形式叫生成器（Generator）
//!    - 性能强，功能弱
//!
//! Rust标准库中支持的协程，就是无栈协程。
