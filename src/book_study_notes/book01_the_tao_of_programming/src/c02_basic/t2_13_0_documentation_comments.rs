//! # 文档注释（documentation comments）
//! Rust文档的哲学是：代码即文档，文档即代码。<br/>
//!
//! **普通注释：**
//! - //          整行注释
//! - /* ... */   整块注释
//!
//! **文档注释：支持Markdown语法，支持示例代码测试，使用rustdoc工具(或cargo doc命令)可以生成HTML文档**
//! - //!         模块文档，一般用于说明整个模块的功能，位置在模块文件的头部。
//! - ///         函数、结构体等文档，一般用于函数和结构体的说明，位置在说明对象的上方。
//!
//!
//! # 打印
//! 在Rust中使用println!()进行打印非常普遍。
//! 其支持的格式如下：
//! Formatting traits
//! When requesting that an argument be formatted with a particular type, you are actually requesting that an argument ascribes to a particular trait. This allows multiple actual types to be formatted via {:x} (like i8 as well as isize). The current mapping of types to traits is:
//!
//! - nothing ⇒ Display
//! - ? ⇒ Debug
//! - x? ⇒ Debug with lower-case hexadecimal integers
//! - X? ⇒ Debug with upper-case hexadecimal integers
//! - o ⇒ Octal
//! - x ⇒ LowerHex
//! - X ⇒ UpperHex
//! - p ⇒ Pointer
//! - b ⇒ Binary
//! - e ⇒ LowerExp
//! - E ⇒ UpperExp
//!
//! 详情查看：[https://doc.rust-lang.org/std/fmt/](https://doc.rust-lang.org/std/fmt/)
