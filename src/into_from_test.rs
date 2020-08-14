//! # From<T> 和 Into<T>
//! std::convert 下面，有两个 Trait，Into/From，它们是一对孪生姐妹。它们的作用是配合泛型，进行一些设计上的归一化处理。
//! 它们的基本形式为： From<T> 和 Into<T>。
//! 实现了From trait就会自动反过来实现Into trait。
//!
//! ## From<T>
//! 对于类型为 U 的对象 foo，如果它实现了 From<T>，那么，可以通过 let foo = U::from(bar) 来生成自己。这里，bar 是类型为 T 的对象。
//! 下面举一例，因为 String 实现了 From<&str>，所以 String 可以从 &str 生成。
//! ```
//! fn test(){
//!     let string = "hello".to_string();
//!     let other_string = String::from("hello");
//!     assert_eq!(string, other_string);
//!  }
//! ```
//!
//! ## Into<T>
//! 如果类型U实现了Into<T>特性, 则类型U的对象foo调用foo.into()会消耗自己（转移资源所有权），生成类型为 T 的另一个新对象 bar。
//! 下面举一例，因为 String 类型实现了 Into<Vec<u8>>，所以 String 可以转换为 Vec<u8> 。
//! ```
//! fn is_hello<T: Into<Vec<u8>>>(s: T) {
//!     let bytes = b"hello".to_vec();
//!     assert_eq!(bytes, s.into());
//! }
//!
//! let s = "hello".to_string();
//! is_hello(s);
//! ```
//!
//! ## 泛型限定示例
//! Into是为了实现类型U转换为类型T.
//! 在入参处指定特性限制, 则可以做到入参必须是实现了可以从自身转到T类型的一个对象,这样无论入参传入什么, 代码内部都可以用其into()方法将其视作T来使用.
//! ```
//! struct Person {
//!     name: String,
//! }
//!
//! impl Person {
//!     fn new<S: Into<String>>(name: S) -> Person {
//!         Person { name: name.into() }
//!     }
//! }
//! ```
//! 这样 name 传入 &str 或 String 类型都可以正常执行,因为他们都实现了Into<String>特性


#[test]
pub fn test_into() {
    let s1 = "111";
    let s2: String = s1.into();
    //to_owned() 把数据从栈中复制到堆中，成为自己的数据
    let s3: String = s1.to_owned();
    println!("{} {} {}", s1, s2, s3);

    let i1: i32 = 1.into();
    println!("{}", i1);
}