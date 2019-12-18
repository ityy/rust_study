//! 流程控制
//! match表达式与模式匹配
//! Rust使用了模式匹配技术，使得匹配条件的编写可以向正则表达式一样灵活。
//! 如果将match当作表达式使用，和if一样，所有分支必须返回同一类型。

/// match 表达式演示
#[test]
fn test_match() {
    let number = 42;
    //关键字 匹配目标
    match number {
        //模式pattern => 匹配后的处理
        //只匹配0
        0 => println!("is 0"),
        //范围匹配
        1..=3 => println!("is 1 to 3"),
        //或
        5 | 6 | 7 => println!("is 5 or 6 or 7"),
        //绑定模式：可以将值绑定到变量 供后面的操作使用
        n @ 42 => println!("is {}", n),
        //通配模式：到此仍未匹配上的，都匹配在这里
        _ => println!("all match"),
    };
}

/// if let 与 while let 表达式
/// 某些只需要单一match的场合，Rust提供了if let 与 while let的简写模式替代match，属于语法糖。
#[test]
fn test_let() {
    //if let 适用于单次匹配判断
    let x = 42;
    let mut binary = 0;
    if let 42 = x {
        binary += 1;
    }
    assert_eq!(binary, 1);

    //while let 适用于多次匹配判断
    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
    /*打印结果
        5
        4
        3
        2
        1
    */
}
