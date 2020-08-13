//! # 结构体 struct
//! **结构体是Rust的主要类型对象。**<br/>
//! **Rust提供三种结构体：**
//! - 具名结构体(Named-Field Struct)
//! - 元组结构体(Tuple-Like Struct)
//! - 单元结构体(Unit-Like Struct)
//!
//! **结构体名称使用大驼峰格式。**


use std::fmt::{Display, Error, Formatter};

/// ## 具名结构体(Named-Field Struct)
/// 最常见，用的最多。
#[derive(Debug, PartialEq)] //宏属性derive: 按默认实现方法,自动实现指定的特性
pub struct Rectangle {
    //成员名: 成员类型
    width: u32,
    height: u32,
}

/// ## 实现一些方法到此结构体
/// 不在impl块下的fn, 称为自由函数。
impl Rectangle {
    /// 仅挂在结构体命名空间下的fn, 称为结构体函数。<br/>
    /// 调用时使用 命名空间::函数名();
    fn new(width: u32, height: u32) -> Self {
        //新建结构体
        return Rectangle {
            //基础赋值方式
            width: width,
            //简洁赋值方式，赋值的变量名与成员名相同, 可以只写变量名
            height,
        };
    }

    /// 挂在结构体命名空间下且第一个参数为self的fn，称为结构体的关联函数（也称为方法）。<br/>
    /// 调用时使用 结构体实例.方法名()； .的一大作用是: 自动传递实例自身到方法。<br/>
    /// 也可使用 命名空间::函数名(对象实例) 用手动传递实例的方式调用方法。
    fn get_width(&self) -> u32 {
        self.width
    }
}

/// ## 实现一些特性到此结构体
/// 手动实现Display特性, 以便支持打印
impl Display for Rectangle {
    /// 覆写特性中的方法
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        print!("width is {}, height is {}", self.width, self.height);
        Ok(())
    }
}


/// ## 元组结构体(Tuple-Like Struct)
/// 特点:字段没有名称 只有类型
struct Color(i32, i32, i32);

/// ## 测试元组结构体
#[test]
fn test_color() {
    let color = Color(111, 222, 333);
    assert_eq!(color.0, 111);
    assert_eq!(color.1, 222);
    assert_eq!(color.2, 333);
}

/// ## New Type模式
/// 当元组结构体只有一个成员时,我们称之为New Type模式.<br/>
/// Integer相当于java的包装类,此时Integer成为了在堆上分配的对象.
struct Integer(u32);

/// ## 使用type定义类型别名
/// type用来定义类型别名。
type Int = i32;


/// ## 测试Integer与Int区别
/// Integer还是按结构体的用法来使用，而Int可以直接使用。
#[test]
fn test_integer_and_int() {
    let int = Integer(32);
    assert_eq!(int.0, 32);
    let int: Int = 10;
    assert_eq!(int, 10);
}

/// ## 单元结构体(Unit-Like Struct)
/// Rust中可以定义一个没有任何成员的结构体
struct Empty;

/// ## 测试单元结构体
// #[test]
pub fn test_empty() {
    let x = Empty;
    println!("{:p}", &x);
    let y = x;
    println!("{:p}", &y);
    let z = Empty;
    println!("{:p}", &z);
    //结果:
    //debug模式下, x,y,z的内存地址均不同, 这是符合逻辑的。
    //release模式下, x,y,z的内存地址均相同，这是编译器检测到单元结构体后人为做的优化。(已运行验证没问题)
    //这表示在debug模式下，单元结构体和普通结构体一样，一个实例占一个堆内存空间。在release模式下，单元结构体的实例会被优化为同一个对象。
}