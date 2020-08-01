//! # 闭包
//! 闭包通常指词法闭包，是一个持有外部环境变量的函数。<br/>
//! 外部环境是指闭包定义时所在的词法作用域。<br/>
//! 外部环境变量，在函数式编程中也被称为自由变量，是指并不在闭包中定义的变量。<br/>
//! 将自由变量和自身绑定的函数就是闭包。<br/>

/// ## 捕获环境变量示例
/// 返回一个实现了Fn(i32) -> i32特性的对象。闭包就可以是这样的对象。
fn counter(i: i32) -> Box<dyn Fn(i32) -> i32> {
    // 实现一个可定义的增加固定步进的函数
    fn inc(s: i32) -> i32 {
        //s + i //error 函数无法捕获外部变量, 只能通过参数传进来
        s + 1 //只能固定写好
    }

    // 如果使用闭包，则可通过捕获环境变量生成闭包。
    // 注：闭包的大小编译器未知，所以用指针包装一下
    Box::new(move |n: i32| n + i)
}

/// ## 闭包的基本语法
#[test]
fn test_basic() {
    let add = |a, b| a + b;
    println!("{}", add(1, 2));
}

/// ## 闭包的实现原理
/// 闭包本质是Rust提供的语法糖，和函数的区别就是可以捕获外部环境变量。<br/>
/// 闭包由编译器来解析转换，也可以通过其它方法来实现闭包。<br/>
/// 实现闭包的核心思想是，通过增加trait将函数调用抽象，变为可重载的操作符。比如将a(b,c,d)这样的函数调用变为如下形式：
/// -   FnOnce::call_once(a,(b,c,d))        对应self 意味着会转移方法接收者的所有权，所以只能调用一次
/// -   Fn::call(&a,(b,c,d))                对应&self 意味着它会对方法接收者不可变借用，可以多次调用
/// -   FnMut::call_mut(&mut a,(b,c,d))     对应&mut self 意味着它会对方法接收者可变借用
///
/// 有了这三个抽象，实现闭包就简单了。我们用struct模拟闭包，用impl这三个trait定义行为。<br/>
/// 每个闭包表达式，实际上都是该闭包结构体的实例，外部环境变量被捕获，放在了结构体的成员内。这一点想起了java，它的lambda表达式也是捕获了外部变量，实际是创建了一个新类，外部变量作为成员变量了。<br/>
/// 知道了此原理，我们就知道为什么闭包可以用特性对象来做类型限定了。<br/>
/// 还可以指定闭包的类型：
#[test]
fn test_principle() {
    // 闭包的类型均不相同,即使如此定义，c1和c2仍为不同的类型。
    let c1 = || {};
    let c2 = || {};

    let env_var = 1;
    // 其实Fn FnOnce FnMut都是特性，其原生写法如下所示：
    // let c1: Box<Fn<(), Output=i32>> = Box::new(|| env_var + 2);
    // 编译器为这三个特性专门做了一种强制性的简便写法：
    let c1: Box<dyn Fn() -> i32> = Box::new(|| env_var + 2);
}


/// ## 闭包与所有权
/// 前面提到，闭包表达式会由编译器自动翻译为结构体实例，并为其实现FnOnce、Fn、FnMut中的一个trait。<br/>
/// 如何知道具体实现了哪一个trait呢？这三个trait和所有权有关。<br/>
/// -   FnOnce      通过转移所有权捕获自由变量，有改变环境的能力，只能调用一次。会消耗自身，对应self。
/// -   Fn          通过不可变借用的方式捕获自由变量，没有改变环境的能力，可以多次调用。对应&self。
/// -   FnMut       通过可变借用的方式捕获自由变量，有改变环境的能力，可以多次调用。对应&mut self。
///
/// 特性父子关系：
/// -   Fn：FnMut        即Fn继承FnMut，要实现Fn必须先实现FnMut
/// -   FnMut：FnOnce    即FnMut继承FnOnce，要实现FnMut必须先实现FnOnce
///
/// 复制语义：自动实现Fn
#[test]
fn test_fn() {
    let s = "hello"; //字符串字面量是值语义、复制语义
    let c = || {
        let x = s;
        println!("{}", x)
    };
    c();
    println!("{}", s);
}

/// 移动语义：自动实现FnOnce
#[test]
fn test_fn_once() {
    let s = "hello".to_string(); //字符串结构体是引用语义、移动语义
    let c = || {
        let x = s;//此处发生移动，如果没有这句直接println!("{}", s)的话，s不会被移动，除非使用move强制移动。
        println!("{}", x)
    };
    c();
    //println!("{}", s);//error: borrow of moved value: `s`
}

/// 强制移动：把闭包捕获的自由变量都移动到闭包内
#[test]
fn test_move() {
    let s = "hello".to_string(); //字符串结构体是引用语义、移动语义
    let c = move || {
        println!("{}", s)
    };
    c();
    //println!("{}", s);//error: borrow of moved value: `s`
}
