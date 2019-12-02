//! 智能指针
//! 智能指针源自C++语言，Rust将其引入，并成为Rust语言中最重要的一种数据结构。
//!
//! 本节只介绍Box指针 。
//! Rust中的值默认被分配到栈内存。可以通过Box<T>将值移动到堆内存（装箱）。
//! 当Box<T>超出作用域时，将调用析构函数，销毁内部对象，并自动释放堆中的内存。

/// 测试Box
#[test]
fn test_box() {
    #[derive(PartialOrd, PartialEq, Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    let box_point = Box::new(Point {
        x: 0.0,
        y: 0.0,
    });
    println!("{:?}", box_point);
    let unboxed_point = *box_point;//value moved here 解引用会导致box_point作废
    println!("{:?}", unboxed_point);
}
