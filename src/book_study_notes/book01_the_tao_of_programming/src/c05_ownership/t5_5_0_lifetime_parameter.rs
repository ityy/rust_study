//! # 生命周期参数
//! 值类型的生命周期与词法作用域有关，但引用可以在各个函数间传递，必然会跨越多个词法作用域。<br/>
//! 对于函数本地声明的拥有所有权的值或者借用来说，Rust编译器包含的借用检查器（borrow checker）可以检查它们的生命周期。<br/>
//! 但是对于跨词法作用域的引用，借用检查器就难以判断合法性了。<br/>
//! 我们需要手动标注生命周期参数，来帮助编译器进行借用检查，以便防止悬垂指针的出现。<br/>
//!
//! # 显式生命周期参数
//! 生命周期参数以单引号开头，标识通常为小写字母，比如'a。<br/>
//! 标注生命周期参数示例：
//! - x:&i32          引用
//! - x:&'a i32       标注生命周期的引用
//! - x:&'a mut i32   标注生命周期的可变引用
//!
//! 注意： <br/>
//!     标注生命周期并不能改变值或引用的生命周期，它只能用于帮助编译器进行借用检查，以便防止悬垂指针的出现。

/// # 函数签名中的生命周期
#[test]
fn test_fn_lifetime() {
    /// 限制：返回值的生命周期长度必须不能大于入参的生命周期长度 <br/>
    /// 禁止在没有任何入参的情况下返回引用，因为这必然带来悬垂指针 <br/>
    fn foo<'a>(s: &'a str, t: &'a str) -> &'a str {
        s
    }

    /// 泛型声明必须在生命周期参数的后面
    fn bar<'a, T>(s: &'a T, t: &'a T) -> &'a T {
        s
    }

    /// 可以标注多个生命周期参数，且指明它们之间的大小关系
    /// ```
    /// fn foobar<'a, 'b>(s: &'a str, t: &'b str) -> &'a str {
    ///     if s.len() > t.len() { s } else { t }
    /// } //error[E0623]: lifetime mismatch  编译器不知道a b的大小关系
    /// ```
    /// 'b: 'a 表示'b 比'a存活的更长。
    /// 返回值不能比入参生命周期长, 标记'b 比'a存活的更长是对的
    fn foobar<'a, 'b: 'a>(s: &'a str, t: &'b str) -> &'a str {
        if s.len() > t.len() { s } else { t }
    }
    /// 测试标记'a 比'b存活的更长，编译器立马报错，违反了返回值不能比入参生命周期长的限制。
    /// ```
    /// fn foobar<'a: 'b, 'b>(s: &'a str, t: &'b str) -> &'a str {
    ///     if s.len() > t.len() { s } else { t }
    /// } //error 返回值的生命周期长度必须不能大于入参的生命周期长度
    /// ```
    fn nothing() {}
}

/// # 结构体与方法中的生命周期
/// 当结构体包含引用类型的成员时，也需要标注生命周期参数。
#[test]
fn test_struct_lifetime() {
    /// # 结构体中的生命周期参数
    /// 限制：结构体的生命周期，应小于等于任意成员的生命周期。否则成员先行析构，则结构体的引用就称为悬垂指针，这是编译器不允许的。
    #[derive(Debug)]
    struct Foo<'a> {
        part: &'a str,
    }

    let words = String::from("hello,world!");
    let first: &str = words.split(',').next().expect("Could not find a ','");
    let foo = Foo {
        part: first,
    };
    println!("{:?}", foo);

    /// # 方法中的生命周期参数
    /// 与泛型类似，需要在impl后面声明，在Foo后面使用。
    impl<'a> Foo<'a> {
        fn new(s: &'a str) -> Self {
            Foo { part: s }
        }
    }
    let second: &str = words.split(',').next().expect("Could not find a ','");
    let foo2 = Foo::new(second);
    println!("{:?}", foo2);
}

/// # 静态生命周期参数
/// Rust有一种特殊的生命周期参数&'static 表示全程有效
#[test]
fn test_static() {
    // 字符串字面量，为全局静态类型。它的值随代码一起被编译进可执行文件内，其地址在编译器可知，且只读。
    // 所以字符串字面量不存在所有权的问题，可以任意使用，因为只读的特性也没有内存安全问题。
    let x = "hello";
    let y = x; //按位复制的是指针，而不是字符串内容。
    println!("{}", x);
    println!("{}", y);
    //2018版Rust中，使用const和static定义字符串字面量时，都可以省略掉'static参数。
}

/// # 生命周期限定
/// 生命周期参数可以和trait一样作为泛型的限定：
/// -   T:'a            T类型的任何引用都要活得和'a一样长
/// -   T:'a + Trait    T类型必须实现Trait，且任何引用都要活得和'a一样长
fn nothing() {}