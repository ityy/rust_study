//! 枚举体
//! 重构上一节：实现输出彩色文本，使其可以输出更多颜色。
//!
//! 枚举体是和类型（Sum Type），表示做一件事有N种方法。其之间的关系为逻辑或。
//! Rust消除空指针的Option就是典型的枚举体,它代表有和无之和。
//! ```
//! pub enum Option<T> {
//!    /// No value
//!    #[stable(feature = "rust1", since = "1.0.0")]
//!    None,
//!    /// Some value `T`
//!    #[stable(feature = "rust1", since = "1.0.0")]
//!    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
//! }
//! ```
//! 拿厕纸盒举例：厕纸盒内是空的，表示None（或null）。厕纸盒内不空，但只剩下一个圆筒心，表示0。
//!
//! 面向对象语言使用继承来复用方法，比如：
//!     class Color
//!     class Red extends Color
//!     class Green extends Color
//!     class Blue extends Color
//! 在Rust中，一个枚举体就够了！



#[test]
fn test_enum() {
    //定义枚举体
    enum Color {
        Red,
        Yellow,
        Blue,
    }

    //实现方法
    impl Color {
        fn to_fg_str(&self) -> &str {
            //*self 只是为了取值，并没有绑定变量发生移动，所以不需要所有权
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

    // 重构结构体
    struct ColoredString {
        input: String,
        fgcolor: Option<Color>,
        bgcolor: Option<Color>,
    }
    impl Default for ColoredString {
        fn default() -> Self {
            ColoredString {
                input: String::default(),
                fgcolor: Option::None,
                bgcolor: Option::None,
            }
        }
    }
}
