//! 变量与绑定
//! 使用 let 关键字创建变量，称为绑定。它表明了标识符和值之间建立的关联关系。
//!
//! 2.3.1 位置表达式和值表达式
//! Rust中表达式分为位置表达式（place expression）和值表达式（value expression）
//! 位置表达式就是内存地址，有如下类型：
//! 1 本地变量
//! 2 静态变量
//! 3 解引用（*expr）
//! 4 数组索引（expr[expr]）
//! 5 字段引用（expr.field）
//! 6 位置表达式组合
//! 除此之外为值表达式。位置表达式是内存地址，可以进行读写操作。读表达式一般只引用了某个存储单元，只能进行读操作。
//! 位置表达式代表持久性数据，值表达式代表临时数据。
//! 值表达式要么是字面量，要么是表达式求值过程中的临时值。
//!
//! 2.3.2
//! 不可变绑定与可变绑定
//! let a=1； //不可变
//! let mut a=1； //可变
//!
//! 2.3.3
//! 所有权与借用
//! let x="hello" //x拥有hello的所有权
//! let y=x; //x的内存地址被转移给y，y拥有hello的所有权。此语义称为move。不转移的那种情况称为copy。
//! println!("{}",&y); //&y称为借用（borrow）操作符，可以获取表达式的内存地址，对此地址进行只读操作。。
//!                    //通过借用操作符得到的类型，称为引用类型（reference）。
#[test]
fn testBorrow() {
    //注意：所有变量标识都本身代表一个内存地址。代码中使用变量标识，默认都是使用这个标识所标识的地址中的值。
    //栈:
    //let [a]=3; a代表0x00000001 [a]为3
    //堆:
    //let [b]=String::new("hello"); b代表0x00000002 [b]为0x10000001 此为hello在堆中的地址。
    //变量是用来储存信息的，所以我们不关心变量标识代表的内存地址（这一块由编译汇编手段来自动完成替换），我们只关心变量标识存储的信息。
    //所以实际编程中，变量标识直接代表其存储的信息，而不代表一个内存地址。相当于省略了[]地址符号，[a]直接写作a。

    let a = [1, 2, 3];//位置表达式a 绑定一个静态长度数组
    let b = &a;//位置表达式b 绑定a的借用 借用操作符'&'取得a的内存地址
    println!("{:p}", b); //{:p}表示打印指针值 0x55fdfeec4 b的值是a的地址，所以按指针打印。注意不是b的地址。


    let mut c = vec![1, 2, 3];//位置表达式c 绑定一个动态长度数组
    let d = &mut c;//位置表达式d 绑定c的可变借用
    d.push(4);
    println!("{:?}", d); //{:?} 按格式打印

    let e = &42; //位置表达式e 绑定&42. 由于42是字面量，是一个只读值。这里编译器会给&42临时创建一个内存地址绑定到e。e的值是42的地址。
    assert_eq!(42, *e); //可以暂时粗暴的认为&为取地址操作，*为从地址取值的操作。
    // 注意：e本身代表e这个内存地址中的值，而e这个地址中存放的是42的地址，所以*e等于42.
}
