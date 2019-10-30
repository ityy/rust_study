//! trait 是Rust的灵魂
//! 有四种用法:
//! 接口抽象 接口是对类型行为的统一约束
//! 泛型约束 泛型的行为被trait限定在更有限的范围
//! 抽象类型
//! 标签trait

use std::ops::Add;

///接口抽象
///trait最基础的用法就是接口抽象
/// 接口中可以定义方法,并支持默认实现
/// 接口不能实现接口, 但可以继承接口
/// 要遵循孤儿规则: trait或目标struct（enum等）至少有一个在当前crate中定义。避免破坏标准库。
///
/// 在Rust中,很多操作符都是trait实现的,比如加法操作符.
///
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    //Add<Rhs=Self> 表示不指定泛型时的默认值
    type Output = Point;//关联类型 通过别名赋值的方式传递类型， 也可以用泛型替代, 推荐前者

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn testPoint() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 5 };
    println!("{:?}", p1 + p2);
}

///接口继承
///表示Add2继承了Add，要实现Add2必须先实现Add
/// <T = Self>表示声明泛型T，默认值等于调用此特性的类型自己
/// 后面的<T>则是使用
pub trait Add2<T = Self>: Add<T> {
    //空代码
}