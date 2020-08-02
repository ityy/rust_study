//! # 结构体实例：实现输出彩色文本
//! 需求：输出文本，且可以设置颜色<br/>
//! 控制台输出文本并附带颜色的方法："\x1B[31;43mHello\x1B[0m"
//! -   \x1B    控制字符的Hex值
//! -   [       控制字符的一部分
//! -   31      31为前景色红色 注意31和43不区分前后，不是靠位置区分前景色背景色的，而是31始终表示前景色红色
//! -   43      43为背景色黄色 43始终表示背景色黄色
//! -   Hello   输出的文本
//! -   \x1B[0m 重置控制信息
//!
use serde::export::fmt::Error;
use serde::export::Formatter;

/// # 结构体 ColoredString
pub struct ColoredString {
    text: String,
    fg_color: String,
    bg_color: String,
}

/// # 给结构体ColoredString增加默认值
/// Rust中给默认值的方式，是实现标准库的Default特性
impl std::default::Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            text: String::default(),
            fg_color: String::default(),
            bg_color: String::default(),
        }
    }
}

/// # 特性 彩色化
/// 定义一个特性接口
pub trait Colorize {
    /*关联常量，类似关联类型，可以给默认值也可以在实现时赋值。使用实现此特性的类型名来调用。*/
    /// 前景色 红色
    const FG_RED: &'static str = "31";
    /// 背景色 黄色
    const BG_YELLOW: &'static str = "43";
    /// 后缀固定格式
    const SUFFIX: &'static str = "\x1B[0m";

    fn set_fg_color(self, color: &str) -> ColoredString;
    fn set_bg_color(self, color: &str) -> ColoredString;
}

/// # 为&str实现彩色化特性
impl<'a> Colorize for &'a str {
    fn set_fg_color(self, color: &str) -> ColoredString {
        ColoredString {
            fg_color: String::from(color),
            text: String::from(self),
            ..ColoredString::default() // 剩余的值使用默认值
        }
    }

    fn set_bg_color(self, color: &str) -> ColoredString {
        ColoredString {
            bg_color: String::from(color),
            text: String::from(self),
            ..ColoredString::default() // 剩余的值使用默认值
        }
    }
}

/// # 为ColoredString实现彩色化特性
/// 为了链式调用，也要给ColoredString实现此特性，方便返回的ColoredString实例继续调用彩色化方法
impl<'a> Colorize for ColoredString {
    fn set_fg_color(self, color: &str) -> ColoredString {
        ColoredString {
            fg_color: String::from(color),
            //省略写法，表示剩下的字段与self相同 注：如果是移动语义，会消耗掉self
            ..self
        }
    }

    fn set_bg_color(self, color: &str) -> ColoredString {
        ColoredString {
            bg_color: String::from(color),
            //省略写法，表示剩下的字段与self相同 注：如果是移动语义，会消耗掉self
            ..self
        }
    }
}

/// # 为ColoredString实现Display特性
impl std::fmt::Display for ColoredString {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // try!() 1.39过时，用？操作替代 ?表示有错就返回Error，无错就获得值，因为此方法的值都是()，可以看出单单是为了利用Result处理错误的功能
        f.write_str(self.get_prefix().as_str())?;
        f.write_str(self.text.as_str())?;
        f.write_str(ColoredString::SUFFIX)?;
        Ok(())
    }
}


/// # 为ColoredString增加方法
/// 输出为附带颜色的最终结果的字符串
impl ColoredString {
    /// 给Display特性中的fmt方法调用
    fn get_prefix(&self) -> String {
        let mut result = String::from("\x1B[");
        let mut has_wrote = false;
        if !self.bg_color.is_empty() {
            result.push_str(&(self.bg_color));
            has_wrote = true;
        }

        if !self.fg_color.is_empty() {
            if has_wrote { result.push(';') }
            result.push_str(&(self.fg_color));
        }
        result.push('m');
        result
    }
}


/// # 抽象与封装的工作完成了，开始测试
#[test]
fn test_struct() {
    let hi = "hello".set_fg_color(ColoredString::FG_RED).set_bg_color(ColoredString::BG_YELLOW);
    println!("{}", hi);

    let hi = "hello".set_bg_color(ColoredString::BG_YELLOW);
    println!("{}", hi);

    let hi = "hello".set_fg_color(ColoredString::FG_RED);
    println!("{}", hi);

    let hi = "hello".set_bg_color(ColoredString::BG_YELLOW).set_fg_color(ColoredString::FG_RED);
    println!("{}", hi);
}