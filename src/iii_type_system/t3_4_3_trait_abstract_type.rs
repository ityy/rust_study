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

//特性限定， 静态派发
fn static_dispatch<T>(t: &T) where T: Bar {
    t.nothing();
}

//特性对象， 动态派发
//直接把实现了Bar的类型的集合作为一个类型
//这部分有点类似java的接口与多态
fn dynamic_dispatch(t: &Bar) {
    t.nothing();
}