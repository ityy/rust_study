//! # let绑定，作用域，生命周期
//! let有let binding的意思。

/// ## 绑定
#[test]
fn test_let() {
    let x = "hello".to_string(); //let绑定了标识符x和存储”hello“的那块内存，从而x对那块内存拥有所有权。
    let _y = x; //那块内存的所有权绑定到了y，x被解绑。这就是移动语义。

    let mut _z = 5;//可变绑定
}


/// ## 生命周期
/// 变量绑定有“时空”双重属性：
/// 1. 空间属性：指标识符与内存空间进行了绑定。
/// 2. 时间属性：指绑定的时效性，即生命周期。
///
/// 生命周期和词法的作用域有关。
#[test]
fn test_scope() {
    let _a = "hello"; //a的生命周期就是test_scope作用域。a作为局部变量，会随着test_scope函数栈帧的销毁而销毁，且编译器知情。
}

