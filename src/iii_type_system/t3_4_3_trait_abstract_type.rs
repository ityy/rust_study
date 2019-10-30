//! trait还可以用于抽象类型

///特性对象
///从类型的维度，各类型可以实现相同的特性
///从特性的维度，实现同一特性的所有类型的集合也可以抽象为一个类型，这就是特性对象（trait object）
///特性限定 与 特性对象的对比
struct Foo;

trait Bar {
    fn nothing(&self);
}

impl Bar for Foo {
    fn nothing(&self) {
        println!("nothing!");
    }
}

//泛型约束， 静态派发
fn static_dispatch1<T>(t: &T) where T: Bar {
    t.nothing();
}

//特性对象， 动态派发
//直接把实现了Bar的类型的集合作为一个类型
//这部分有点类似java的接口与多态
fn dynamic_dispatch1(t: &Bar) {
    t.nothing();
}
//Bar是一个特性,特性对象是抽象的对象,其大小无法确定,所以可以用&Bar或Box<T>来制造一个trait对象
//要成为特性对象,有诸多限制
//1 受Self:Sized约束
//2 第一个参数为self
//3 没有泛型 (否则特性对象不知道调哪个方法)


///impl Trait
///2018版新加入的 静态派发抽象类型
/// 可以部分替代泛型约束,使代码清爽
/// 性能比使用特性对象更好
fn static_dispatch2(t: impl Bar) -> impl Bar {
    t.nothing();
    t
}

///dyn Trait
///2018版新加入的,相对于impl Trait来说,为特性对象新加的语法:动态派发抽象类型
/// Box<dyn Bar>其实就是特性对象,2015版写作Box<Bar>,新语法为了突出其动态派发的性质
fn dynamic_dispatch2(t: &Bar) -> Box<dyn Bar> {
    t.nothing();
    Box::new(t)
}

