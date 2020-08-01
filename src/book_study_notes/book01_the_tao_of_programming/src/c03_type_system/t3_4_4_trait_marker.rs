//! # trait 系统：标签 marker
//! 可以利用trait约束的功能给类型打标签，Rust提供了5个重要的标记trait：
//! -   Sized       标识编译器可以确定大小的类型
//! -   Unsized     标识编译器不可以确定大小的类型，即动态大小类型
//! -   Copy        标识可以安全的按位复制的类型
//! -   Send        标识可以安全的跨线程通信的类型  即安全传值(含所有权)
//! -   Sync        标识可以安全的在线程间共享引用的类型  即安全传引用(不含所有权)
//!
//! Rust正是有了这种标记机制，可以将许多以前的编程语言中固定的一些法则，在Rust中使用标记进行明确。<br/>
//! - 比如Copy标记，明确了栈上复制的类型。
//! - 比如Hash标记，明确了可以进行Hash计算从而作为Key使用的类型。
//!
//! 这些在很多传统的编程语言中，都是作为语法规则或文档进行描述，由开发人员自行保证编写正确的代码。
//! Rust通过特性标记，生命周期标记等手段，帮助编译器进行安全检查，以在编译阶段就可以发现内存安全问题和线程安全问题。
//! 这就是Rust的设计目标，由编译器来确保非unsafe状态下的内存和线程安全。由Rust语言的开发者来预先标记区分。
//! 相当于由语言设计者来保证开发人员可以编写正确的代码。心智负担由开发人员转移到语言设计人员身上。
//! 有时这些标记也可能漏标或者错标而带来bug，Rust语言的迭代过程中都会修复这些问题。