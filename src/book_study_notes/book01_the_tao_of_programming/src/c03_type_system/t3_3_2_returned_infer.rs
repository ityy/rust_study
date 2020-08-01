//! # 泛型返回值自动推导
//! 声明两个结构体Foo、Bar 和一个特性X，使Foo、Bar实现X。<br/>
//! 声明一个测试函数foobar，返回值是X类型。<br/>
//! 则调用此函数时，可以根据绑定变量的实际类型推断出返回值的实际类型。

//声明一个元组结构体
#[derive(Debug, PartialEq)]
struct Foo(i32);

//声明一个元组结构体
#[derive(Debug, PartialEq)]
struct Bar(i32, i32);

//声明一个特性
trait X {
    fn new(i: i32) -> Self;
}

//为结构体实现特性
impl X for Foo {
    fn new(i: i32) -> Self {
        Foo(i)
    }
}

impl X for Bar {
    fn new(i: i32) -> Self {
        Bar(i, i * 2)
    }
}

//声明一个函数, 并限定返回值的泛型类型 必须实现Inst特性
fn foobar<T: X>(i: i32) -> T {
    T::new(i)
}

#[test]
fn test() {
    //必须标注f的类型, foobar就可以根据该类型推导出要调用的结构体
    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));
}