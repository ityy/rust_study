//! # 枚举体 enum
//! 顾名思义，该类型包含了全部可能的情况，可以有效的防止用户提供无效值。
//! ## Rust提供三种枚举体
//!  - 无参数枚举体
//!  - c语言枚举体
//!  - 有参数枚举体
//!
//! 枚举是Rust非常重要的类型，Rust利用枚举实现了很多约束，比如Option、Result等。

/// ## 1 无参数枚举体
#[test]
fn no_params_enum() {
    enum Number {
        //枚举体定义了3个值
        Zero,
        One,
        Two,
    }
    let a = Number::One;
    match a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    }
}

/// ## 2 c语言枚举体
/// Rust可以像c语言一样编写枚举体，使枚举体的值更像是一个常量。
#[test]
fn c_enum() {
    enum Color {
        Red = 0xFF0000,
        Green = 0x00FF00,
        Blue = 0x0000FF,
    }
    println!("roses are #{:06X}", Color::Red as u32);
    println!("violets are #{:06X}", Color::Blue as u32);
    /*结果：
        roses are #FF0000
        violets are #0000FF
     */
}

/// ## 3 有参数枚举体
#[test]
fn params_enum() {
    #[derive(Debug)]
    enum IpAddr {
        //这样的枚举值本质上是使用函数来实现的，可以通过显式的指定类型来转换为函数指针。
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);

    //枚举值与函数指针验证
    let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4; //注意不是IpAddr::V4()。
    let lan = x(192, 168, 1, 1);

    println!("{:?}", home);
    println!("{:?}", lan);
    /*打印结果
        V4(127, 0, 0, 1)
        V4(192, 168, 1, 1)
    */
}
