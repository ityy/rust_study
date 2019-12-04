//! 通用概念
//! 很多编程语言中的数据类型可以分为两部分：
//!     值类型（value）：数据在变量中
//!     引用类型（Reference）：数据在堆内存中，数据的地址在变量中
//! 随着语言的发展，越来越多的复合类型使得这一划分已经不能满足。于是有了新的描述：
//!     值语义（value semantic）：复制以后，两个对象拥有的存储空间是独立的，互不影响。
//!     引用语义（Reference semantic）：复制以后，两个对象只是相同存储空间的别名，修改一个也会影响另一个。
//! Rust使用Copy特性作为标记，将类型按值语义和引用语义进行了精确分类。
//!
//! 所有权机制
//! 在Rust中，可以通过尝试是否能实现Copy特性，来确认数据类型是值语义还是引用语义。
//!     对值语义，则是按位复制。
//!     对引用语义，则是移动所有权。
//! Rust中每个值都有唯一的一个所有者，负责该内存的读写和释放。
//! 所有权的特点：
//!     控制资源的释放
//!     出借所有权（不可变共享的，可变独占的）
//!     转移所有权
//!
#[test]
fn test_move() {
    //Box将值放在了堆中，且无法实现Copy。具有引用语义。
    let x = Box::new(5);
    //引用语义会移动所有权。
    let y = x;
    //println!("{:?}", x);//error: value borrowed here after move
}

/// Copy标记
/// 当元素都为Copy时，容器可以为Copy。
/// 比如结构体structA，元素都为Copy型的，则structA也可以实现Copy。
/// 但不会自动实现，需要手动加上derive。
/// 元组的元素如果都为Copy型，则元组会自动实现Copy。
#[derive(Debug, Copy, Clone)]
struct structA {
    a: i32,
    b: i32,
}

#[test]
fn test_copy() {
    let a = structA { a: 1, b: 2 };
    let b = a;
    println!("{:?}", a); //run is OK
}
