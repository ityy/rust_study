//! # trait 系统：接口抽象
//! trait 是Rust的灵魂，有四种用法:
//! - 接口抽象      接口是对类型行为的统一约束
//! - 泛型限定      泛型的行为被trait限定在更有限的范围
//! - 抽象类型
//! - 标签        可以使用trait打标签
//!
//! ## 接口抽象
//! trait 最基础的用法就是接口抽象。
//! - 接口中可以定义方法，并支持默认实现。
//! - 接口不能实现接口, 但可以继承接口
//! - 要遵循孤儿规则: trait或目标struct（enum等）至少有一个在当前crate中定义。避免破坏标准库。
use std::fmt::Display;
use std::ops::Add;

/// ## 示例：接口实现
/// 在Rust中,很多操作符都是 trait 实现的，比如加法操作符。<br/>
/// 本示例为结构体实现Add接口，使结构体具有加法功能。<br/>
///
/// 1 创建结构体
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

/// 2 实现加法接口
impl Add<Point> for Point {
    //Add<Rhs=Self> 表示不指定泛型时的默认值
    type Output = Point;//关联类型：通过别名赋值的方式传递类型，也可以用泛型替代，推荐前者。

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// 3 测试结构体相加
#[test]
fn test_point_add() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 5 };
    println!("{:?}", p1 + p2);
}

/// ## 示例：接口继承
/// 在定义trait时，Rust使用冒号来表达继承关系。<br/>
/// 表示Add2继承了Add，要实现Add2必须先实现Add。<br/>
/// <T = Self> 表示声明泛型T，默认值等于调用此特性的类型自己。再将T传递给父接口。
pub trait Add2<T = Self>: Add<T> {
    //空代码
}

/// # 为指定类型实现特性
#[test]
pub fn test_disyang() {
    //因为整型实现了Display特性, 所以其也实现了DisYang特性
    3.dis_yang();

    //因为字符串实现了Display特性, 所以其也实现了DisYang特性
    "测试".dis_yang();

    /*打印结果:
        this is yang trait!
        this is yang trait!
     */
}

/// 自定义特性
trait DisYang {
    //直接给出默认实现
    fn dis_yang(&self) {
        println!("this is yang trait!");
    }
}

/// 给所有实现了Display特性的对象 也实现DisYang特性
impl<T: Display> DisYang for T {}

