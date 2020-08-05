//! # 名字、标识符、变量
//! 虽然术语“名字”和“变量”通常指的是同一个事物,我们还是要很小心地使用它们,以便区别编译时刻的名字和名字在运行时刻所指的内存位置。  <br/>
//!
//! 标识符(identifier)是一个字符串,通常由字母和数字组成。它用来指向(标记)一个实体,比如一个数据对象、过程、类,或者类型。 <br/>
//! 所有的标识符都是名字,但并不是所有的名字都是标识符。 <br/>
//! 名字也可以是一个表达式。比如名字x.y可以表示x所指的一个结构中的字段y.这里,c和y是标识符,而x.y是一个名字。 <br/>
//! 像x.y这样的复合名字称为受限名字(qualifiecname) <br/>
//!
//! 变量指向存储中的某个特定的位置。 <br/>
//! 同一个标识符被多次声明是很常见的事情,每一个这样的声明引入一个新的变量。 <br/>
//! 即使每个标识符只被声明一次,一个递归过程中的局部标证符将在不同的时刻指向不同的存储位置。 <br/>

use crate::memory_and_pointer_test::memory_operation::MemoryOperation;

/// # 谁申请的栈内存
/// 是标识符表示栈内存然后放入了值，还是值占用了栈内存然后绑定到了标识符上？ <br/>
/// 前者表示多个不同的标识符就是多个不同的栈地址，后者表示相同值的多个标识符就是一个栈地址。 <br/>
/// 答案显然是前者。
/// - rust中的let相当于创建新的栈内存标识符，即使重名（重名时旧变量被隐藏shadowing）。
/// - java中在变量声明时相当于创建新的栈内存标识符，重名不可再次声明。
#[test]
fn who_create_stack_address() {
    // let 创建一个名为name的栈内存，地址为&name，值为指向hello的String指针，则name是一个String智能指针。
    let mut name = String::from("hello");
    println!("name addr:{:p}", &name);// 0x238f9ff0d0
    println!("name value:");
    MemoryOperation::print_addr_and_value(MemoryOperation::convert_addr_to_int(&name), 16);

    // 为name重新赋值，栈内存name的地址固定，值发生了变化。值为指向"world"的String指针。
    name = String::from("world");
    println!("name addr:{:p}", &name);// 0x238f9ff0d0
    println!("name value:");
    MemoryOperation::print_addr_and_value(MemoryOperation::convert_addr_to_int(&name), 16);

    // let 创建了一个名为name的栈内存（旧的栈内存name被隐藏 shadowing）
    let mut name = String::from("rust!");
    println!("name addr:{:p}", &name);// 0xdf148ff378
    println!("name value:");
    MemoryOperation::print_addr_and_value(MemoryOperation::convert_addr_to_int(&name), 16);
}


/// # 找回被隐藏的资源
/// 原理：利用std::mem::replace交换两个变量的资源
/// ```
/// pub fn replace<T>(dest: &mut T, mut src: T) -> T {
///     swap(dest, &mut src);
///     src
/// }
/// ```
#[test]
fn find_shadowed_resource() {
    let mut string_owner = String::from("hello");
    println!("hello 堆中地址：{:p}", string_owner.as_ptr());// 0x204a5ac5450
    let old_string_ref = &mut string_owner;

    // 发生遮蔽：string_owner 相当于一个新变量，旧资源的所有者 string_owner（旧） 处于隐藏状态
    let mut string_owner = String::from("world");

    // 找回所有权
    // 利用 std::mem::replace 交换两个变量的资源:
    //      old_string_owner 获得了 hello 的所有权
    //      old_string_ref 成为了 swap 的独占借用，swap 的所有者 string_owner（旧） 处于隐藏状态
    let old_string_owner = std::mem::replace(old_string_ref, String::from("swap"));
    println!("hello 堆中地址：{:p}", old_string_owner.as_ptr());// 0x204a5ac5450 堆中地址一致，资源被找回
    println!("old_string_ref：{}", old_string_ref);// swap
}

