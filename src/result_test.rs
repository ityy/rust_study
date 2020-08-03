//! # Result<T, E> 解析
//! Result<T, E> 是Rust内置的一个枚举，含有Ok(T)、Err(E) 两个元素。
//! ```
//! pub enum Result<T, E> {
//!     /// Contains the success value
//!     #[stable(feature = "rust1", since = "1.0.0")]
//!     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
//!
//!     /// Contains the error value
//!     #[stable(feature = "rust1", since = "1.0.0")]
//!     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
//! }
//! ```
//! 可以将其按普通枚举自由使用。
//! Rust为Result<T, E>实现了一套方法，用于快捷操作。

/// # T、E均指定为u32
fn err_u32() -> Result<u32, u32> {
    // Ok(1)
    Err(2)
}


/// # T指定为u32，E指定为String
fn err_string() -> Result<u32, String> {
    // Ok(1)
    Err("error message".into())
}


#[test]
fn test() {
    let result = err_u32().unwrap();//thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: 2', src\main.rs:32:18
    println!("result:{}", result);
    let result = err_string().unwrap();//thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "error message"', src\main.rs:34:18
    println!("result:{}", result);
}