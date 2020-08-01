//! # 结构体
//! 实现输出彩色文本

use serde::export::fmt::Error;
use serde::export::Formatter;

/// ## 需求：输出文本，且可以设置颜色
/// 控制台输出文本并附带颜色的方法："\x1B[31;43mHello\x1B[0m"
/// -   \x1B    控制字符的Hex值
/// -   [       控制字符的一部分
/// -   31      31为前景色红色 注意31和43不区分前后，不是靠位置区分前景色背景色的，而是31始终表示前景色红色
/// -   43      43为背景色黄色 43始终表示背景色黄色
/// -   Hello   输出的文本
/// -   \x1B[0m 重置控制信息
#[test]
fn test_struct() {
    //结构体 封装需求
    struct ColoredString {
        input: String,
        fgcolor: String,
        bgcolor: String,
    }

    //给结构体ColoredString增加默认值
    //Rust中给默认值的方式，是实现标准库的Default特性
    impl std::default::Default for ColoredString {
        fn default() -> Self {
            ColoredString {
                input: String::default(),
                fgcolor: String::default(),
                bgcolor: String::default(),
            }
        }
    }

    //特性 彩色化
    trait Colorize {
        //关联常量，类似关联类型，可以给默认值也可以在实现时赋值。使用实现此特性的类型名来调用。
        const FG_RED: &'static str = "31";
        const BG_YELLOW: &'static str = "43";

        fn red(self) -> ColoredString;
        fn on_yellow(self) -> ColoredString;
    }

    // 为字符串实现彩色化特性，返回ColoredString实例
    impl<'a> Colorize for &'a str {
        fn red(self) -> ColoredString {
            ColoredString {
                fgcolor: String::from(ColoredString::FG_RED),
                input: String::from(self),
                ..ColoredString::default()
            }
        }

        fn on_yellow(self) -> ColoredString {
            ColoredString {
                bgcolor: String::from(ColoredString::BG_YELLOW),
                input: String::from(self),
                ..ColoredString::default()
            }
        }
    }

    //为了链式调用，也要给ColoredString实现此特性，方便返回的ColoredString实例继续调用彩色化方法
    impl<'a> Colorize for ColoredString {
        fn red(self) -> ColoredString {
            ColoredString {
                fgcolor: String::from(ColoredString::FG_RED),
                //省略写法，表示剩下的字段与self相同 注：如果是移动语义，会消耗掉self
                ..self
            }
        }

        fn on_yellow(self) -> ColoredString {
            ColoredString {
                bgcolor: String::from(ColoredString::BG_YELLOW),
                //省略写法，表示剩下的字段与self相同 注：如果是移动语义，会消耗掉self
                ..self
            }
        }
    }

    //输出为附带颜色的最终结果的字符串
    impl ColoredString {
        fn compute_style(&self) -> String {
            let mut res = String::from("\x1B[");
            let mut has_wrote = false;
            if !self.bgcolor.is_empty() {
                res.push_str(&(self.bgcolor));
                has_wrote = true;
            }

            if !self.fgcolor.is_empty() {
                if has_wrote { res.push(';') }
                res.push_str(&(self.fgcolor));
            }
            res.push('m');
            res
        }
    }

    //实现Display特性，以控制输出效果
    impl std::fmt::Display for ColoredString {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            let mut input = self.input.clone();
            // try!() 1.39过时，用？操作替代 ?表示有错就返回Error，无错就获得值，因为此方法的值都是()，可以看出单单是为了利用Result处理错误的功能
            f.write_str(self.compute_style().as_str())?;
            f.write_str(self.input.as_str())?;
            f.write_str("\x1B[0m")?;
            Ok(())
        }
    }

    // 抽象与封装的工作完成了，开始测试
    let hi = "hello".red().on_yellow();
    println!("{}", hi);
    let hi = "hello".on_yellow();
    println!("{}", hi);
    let hi = "hello".red();
    println!("{}", hi);
    let hi = "hello".on_yellow().red();
    println!("{}", hi);
    //效果完美！
}