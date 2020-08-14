//! # pattern 模式


/// # 解构会move所有权
#[test]
fn test1() {
    let tuple: (u32, String) = (5, String::from("five"));

    let (x, s) = tuple;

    // 以下行将导致编译错误，因为String类型并未实现Copy, 所以tuple被整体move掉了。
    // println!("Tuple is: {:?}", tuple);
}

/// # 忽略匹配的字段将不会被move
#[test]
fn test2() {
    let tuple = (5, String::from("five"));

    // 忽略String类型，而u32实现了Copy，则tuple不会被move
    let (x, _) = tuple;

    println!("Tuple is: {:?}", tuple);
}