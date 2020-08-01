//! # 流程控制
//! 一般编程语言都有流程控制语句：条件语句和循环语句。Rust也不例外，但Rust叫：条件表达式，循环表达式。表达式可以返回值。

/// ## if 表达式
#[test]
fn test_if() {
    let n = 13;
    let big_n = if n < 10 && n > -10 {
        10 * n
    } else {
        n / 2
    };//编译器根据可能的返回类型 统一推断为某一特定类型。因为10 * n必为整数，所以n/2不是浮点数，而是整数
    assert_eq!(6, big_n);
}

/// ## for 循环表达式
#[test]
fn test_for() {
    //
    for i in 0..100 {
        println!("{}", i);
    };

    //遍历迭代器
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

/// ## while 循环表达式
#[test]
fn test_while() {
    let mut i = 100;
    while i > 0 {
        println!("{}", i);
        i = i - 1;
    }
}

/// ## loop 表达式
/// 无限循环，是循环表达式的子集。比while（true）好的一点是，编译器看到loop就知道是无限循环，可以做一些优化。
#[test]
fn test_loop() {
    let mut i = 100;
    loop {
        if i > 0 { return; }
        println!("{}", i);
        i = i - 1;
    }
}
