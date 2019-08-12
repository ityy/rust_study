//! std::convert 下面，有两个 Trait，Into/From，它们是一对孪生姐妹。它们的作用是配合泛型，进行一些设计上的归一化处理。
//!
//! 它们的基本形式为： From<T> 和 Into<T>。
//! From
//! 对于类型为 U 的对象 foo，如果它实现了 From<T>，那么，可以通过 let foo = U::from(bar) 来生成自己。这里，bar 是类型为 T 的对象。
//!
//! 下面举一例，因为 String 实现了 From<&str>，所以 String 可以从 &str 生成。
//!
//! fn test(){
//!  let string = "hello".to_string();
//!  let other_string = String::from("hello");
//!  assert_eq!(string, other_string);
//!  }

pub fn main() {
    let string = "hello".to_string();
    let other_string = String::from("hello");

    assert_eq!(string, other_string);
}