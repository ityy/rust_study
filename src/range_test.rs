//! Range时标准库里的一个结构体
//! 有两个属性, start和end
//! rust提供了一个语法糖, 可以快速创建range
//! 只需要使用(x..y)的形式, 即可实现一个[x,y)的左闭右开的区间
use std::ops::Range;
use std::ops::RangeInclusive;

pub fn main() {
    //语法糖 快速创建Range
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    //Range相关的方法
    assert_eq!((3..6).max().unwrap(), 5);
    //此版rust下列方法报错 sum 是Iterator trait的一个方法
//    assert_eq!((3..=6).sum(), 3 + 4 + 5 + 6);
}