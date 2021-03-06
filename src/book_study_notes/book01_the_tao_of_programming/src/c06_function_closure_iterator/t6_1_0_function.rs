//! # 函数

use std::ops::Mul;

/// # 函数
/// 函数名需要遵循蛇形命名法(snake_case)
/// 函数参数等价于一个隐式的let绑定，所以函数参数也支持模式匹配，比如（x,y）:(i32,i32)来解构元组。
/// Rust的函数有且只有一个返回值，即使没有显式的支持，函数仍会默认返回一个()单元值。
fn test_fn(x: i32) {//函数签名
    //函数体
    println!("{}", x);
}

/// # 泛型函数
/// 实现泛型函数，可以节约工作量。
#[test]
fn test_generic() {
    /// ## 求积
    /// 使用泛型限定，以限制只有可以被乘的类型才可以调用此函数
    fn square<T: Mul<T, Output=T>>(x: T, y: T) -> T {
        x * y
    }

    //默认f64
    println!("{}", square(111.2, 333.5));
    //使用turbo fish操作符指定类型
    println!("{}", square::<f32>(111.2, 333.5));
    /*结果：
    37085.200000000004
    37085.2
     */
}