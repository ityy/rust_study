//!derive 是一种Procedural宏, 可以理解为获得
//! #[derive(特性1,特性2...)] 注解在结构体上, 将按derive宏定义好的方式, 为此结构体生成实现指定特性的代码.

///定义一个特性
pub trait HelloMacro {
    fn hello_macro();
}

