/// 结构体学习
use std::fmt::{Display, Error, Formatter};

struct Rectangle {
    width: u32,
    height: u32,
}

pub fn mainT() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "the area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect is {}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/// 实现此特性, 以便支持打印
impl Display for Rectangle {
    /// 实现特性(即覆写方法,实现接口)
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        print!("width is {}, height is {}", self.width, self.height);
        Result::Ok(())
    }
}

