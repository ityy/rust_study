//! # trait 系统：抽象类型
//! - 从类型的维度，各类型可以实现相同的特性
//! - 从特性的维度，实现同一特性的所有类型的集合也可以抽象为一个类型，这就是特性对象（trait object）


/// ## 特性限定 与 特性对象的对比
/// 前者静态派发，属于无消耗抽象。后者动态派发，有运行时消耗。
struct Foo;

trait Bar {
    fn nothing(&self);
}

impl Bar for Foo {
    fn nothing(&self) {
        println!("nothing!");
    }
}

/// ## 泛型约束， 静态派发
fn static_dispatch<T>(t: &T) where T: Bar {
    t.nothing();
}

/// ## 特性对象， 动态派发
/// 直接把实现了Bar的类型的集合作为一个类型，这部分有点类似java的接口与多态。<br/>
/// Bar是一个特性, 其大小无法确定, 可以用&Bar（普通指针）或Box<Bar>（智能指针）来传递特性对象。<br/>
/// dyn 说明：
/// - dyn是2018版引入的一个新的规范，要求特性对象都要用dyn标记出来，以突出其动态派发的性质。<br/>
/// - 在2018版之后，指针应为&dyn Bar 或 Box<dyn Bar>。<br/>
///
/// 要成为特性对象,有诸多限制：
/// 1. 受Self:Sized约束
/// 2. 第一个参数为self
/// 3. 没有泛型 (否则特性对象不知道调哪个方法)
fn dynamic_dispatch(t: &dyn Bar) -> Box<&dyn Bar> {
    t.nothing();
    Box::new(t)
}


/// ## 替代特性对象的新版语法 impl Trait
/// 2018版新加入的，静态派发抽象类型。可以部分替代泛型约束,使代码更简洁。性能比使用动态分发的特性对象更好。
fn static_dispatch_new(t: impl Bar) -> impl Bar {
    t.nothing();
    t
}
