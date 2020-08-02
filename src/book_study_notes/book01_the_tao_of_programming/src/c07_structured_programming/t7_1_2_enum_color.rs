//! # 枚举体实例：实现输出更多颜色的文本
//! 重构上一节：实现输出彩色文本，使其可以输出更多颜色。<br/>
//!
//! 枚举体是'和类型（Sum Type）'，表示做一件事有N种方法。元素之间的关系为逻辑或。<br/>
//! Rust消除空指针的Option就是典型的枚举体,它代表 '有' 和 '无' 之和。
//! ```
//! pub enum Option<T> {
//!    /// No value
//!    #[stable(feature = "rust1", since = "1.0.0")]
//!    None,
//!    /// Some value `T`
//!    #[stable(feature = "rust1", since = "1.0.0")]
//!    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
//! }
//! // 拿厕纸盒举例：厕纸盒内是空的，表示None（null、nil等）。厕纸盒内不空，但只剩下一个空纸筒心，表示0。
//! ```
//! 面向对象语言使用继承来复用方法，比如：
//! -   class Color
//! -   class Red extends Color
//! -   class Green extends Color
//! -   class Blue extends Color
//!
//! 在Rust中，一个枚举体就够了！

/// ## 导入说明：
/// 同crate下的不同模块内，pub 权限的结构体、函数等才能跨模块引用。两种引用方式：
/// - 在使用处直接全路径引用crate::xxx::xxx
/// - 使用use crate::xxx::xxx导入命名空间
///
/// trait有一个注意点：需要使用use显式引入特性，才能让自定义实现特性的目标在本模块下也生效。
/// 本例中是让impl<'a> Colorize for &'a str在本模块也生效，所以要显式引入特性：
use crate::c07_structured_programming::t7_1_1_struct_color::Colorize;

/// 定义枚举体
enum Color {
    Red,
    Yellow,
    Blue,
}

/// 为枚举实现方法
impl Color {
    fn to_fg_str(&self) -> &str {
        //*self 只是为了取值，并没有绑定变量发生移动，所以没有触发所有权转移，所以不会报错
        match *self {
            Color::Red => "31",
            Color::Yellow => "33",
            Color::Blue => "34",
        }
    }
    fn to_bg_str(&self) -> &str {
        match *self {
            Color::Red => "41",
            Color::Yellow => "43",
            Color::Blue => "44",
        }
    }
}


#[test]
fn test_enum() {
    let hi = "hello".set_fg_color(Color::Red.to_fg_str()).set_bg_color(Color::Yellow.to_bg_str());
    println!("{}", hi);

    let hi = "hello".set_bg_color(Color::Blue.to_bg_str());
    println!("{}", hi);

    let hi = "hello".set_fg_color(Color::Yellow.to_fg_str());
    println!("{}", hi);

    // 查看hi的完整类型
    // 方式一： 使用编译器报错的形式，让编译器告知完整类型：
    // let () = hi;
}
