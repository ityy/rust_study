///option枚举测试
/// 主要用于解决null的问题, 提供一个some,一个none
pub fn mainT() {
    let some_number = Some(5);
    let some_string = Some("is a string");
    let absent_number: Option<i32> = None;

    let x = Some(1);
    let y = None;
    let xy = match x {
                    Some(x) => x,
                    None => 0,
                }
                +
                match y {
                    Some(y) => y,
                    None => 0,
                };
    println!("{}",xy);
}