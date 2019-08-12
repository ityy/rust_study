//! Into
//! 如果类型U实现了Into<T>特性, 则类型U的对象foo调用foo.into()会消耗自己（转移资源所有权），生成类型为 T 的另一个新对象 bar。
//!
//! fn is_hello<T: Into<Vec<u8>>>(s: T) {
//!     let bytes = b"hello".to_vec();
//!     assert_eq!(bytes, s.into());
//! }
//!
//! let s = "hello".to_string();
//! is_hello(s);
//! 因为 String 类型实现了 Into<Vec<u8>>。
//!


//! Into是为了实现类型U转换为类型T.
//! 在入参处指定特性限制, 则可以做到入参必须是实现了可以从自身转到T类型的一个对象,这样无论入参传入什么, 代码内部都可以用其into()方法将其视作T来使用.
//! struct Person {
//!     name: String,
//! }
//!
//! impl Person {
//!     fn new<S: Into<String>>(name: S) -> Person {
//!         Person { name: name.into() }
//!     }
//! }
//!
//! 这样name传入&str或String类型都可以正常执行,因为他们都实现了Into<String>特性


fn is_hello<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

pub fn main() {
    let s = "hello".to_string();
    is_hello(s);// 因为 String 类型实现了 Into<Vec<u8>>。
}