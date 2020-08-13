//! # 闭包 closure
//! ## 闭包的特点：
//! 1. 可以像匿名函数一样被调用
//! 2. 可以捕获上下文环境中的自由变量 (而函数不可以）
//! 3. 可以自动推断输入和返回的类型
//!
//! ## Rust中闭包的原理：
//! Rust中的闭包是由一个匿名结构体和trait组合实现的。本质上是一个实现了指定特性的结构体。了解本质，对闭包的使用会更加清晰明了。

/// ## 闭包的声明
#[test]
fn test_closure() {
    let out = 42;
    // 函数捕获out报错：
    //fn add(i: i32, j: i32) -> i32 { i + j + out } //error[E0434]: can't capture dynamic environment in a fn item 无法在fn项中捕获动态环境

    //闭包 带类型注解
    let closure_annotated = |i: i32, j: i32| -> i32{ i + j + out };//使用外部变量
    //闭包 不带类型注解,可以自动推断
    let closure_inferred = |i, j| i + j + out;

    //调用闭包
    assert_eq!(45, closure_annotated(1, 2));
    assert_eq!(45, closure_inferred(1, 2));
}


/// ## 闭包作为参数
#[test]
fn closure_as_parameter() {
    fn math<F>(op: F) -> i32
    //泛型F 受Fn()->i32特性限制
    //注意：约束泛型的是Fn特性，fn可以作为类型注解，但不能约束泛型。
        where F: Fn() -> i32 {
        op()
    }

    assert_eq!(3, math(|| 1 + 2));
    assert_eq!(2, math(|| 1 * 2));
}

/// ## 闭包作为返回值
#[test]
fn closure_as_returned() {
    //返回闭包 也就是返回实现了 “impl Fn(i32) -> i32” 特性的结构体 细节由编译器实现
    fn two_times_impl() -> impl Fn(i32) -> i32 {
        let i = 1;
        //声明一个闭包
        //move表示将被捕获变量的所有权移动给闭包
        move |j| j + i
    }

    let closure_ = two_times_impl();
    assert_eq!(3, closure_(2));
}



