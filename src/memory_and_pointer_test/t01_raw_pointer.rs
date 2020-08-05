//! # 变量、指针、栈内存、堆内存
//!
//! 一个数据在内存中，有地址（address）和值（value）两个属性:
//! ```
//! ----------
//! | address  |
//! ----------
//! |  value   |
//! ----------
//! ```
//!
//! ## 变量的本质：
//! 变量的三个维度：
//! - 变量类型，对指定内存区块使用方式的描述：占了多少个字节，以什么方式来使用它。
//! - 变量地址，变量标识符所代表的内存地址，一般用 &变量标识符 取得变量首地址。
//! - 变量值，变量地址中存放的按变量类型描述的值，一般 变量标识符 就代表变量的值本身。
//!
//! 以汇编为例：<br/>
//! ```mov [0x00001111],1``` 表示向内存地址0x00001111处写入1。<br/>
//! 一般高级编程语言中，我们用 变量标识符 来替代汇编语言中的 [0x00001111] 符号，以表达某个内存地址，可读可写。<br/>
//! 例如一个变量a，通常a表示[0x00001111]，它是一个地址空间，可读可写。<br/>
//! 严格来讲，对于局部变量，是[ebp+xxx]的xxx 即ebp的偏移地址，对于全局变量，则是标号（）。<br/>
//! 详细说明：
//! ```
//! 变量地址：&操作符可以用来获取变量的地址，&a表达的便是a的地址0x00001111。由于变量都是在栈上存放，所以‘&变量’取到的地址一般都是栈上的地址。
//! 变量的使用原则：我们对一切变量的使用，都是从值的角度来使用的。只有&操作符存在时，我们才关注变量自身的内存地址。
//! 数据类型：是对一段数据的两大描述：占了多少个字节，以什么方式来使用它。
//!         编程语言预置了一些数据类型，我们也可以自定义数据类型。我们在计算机的历史上不断的增加各种使用方式。
//!             int型：   4字节                 我们按数值的方式使用它
//!             char型：  1字节                 我们按符号的方式使用它
//!             指针型：   64字节（64bit主机）    我们按内存地址的方式使用它
//!                                             指针型也需要指定数据类型，用于描述以什么类型解析此内存地址
//!                                             指针型权限太大，一些编程语言屏蔽了它，只开放了引用型，即受限制的指针。
//!             ...
//! 变量类型：即变量中存储的数据的数据类型。强类型语言中，某类型的变量只能存放某类型的数据。
//! ```
//! ## 指针的本质：
//! 指针就是一种数据类型，表达的是内存地址类型。不像汇编中可以直接拿一个数值比如0x00001111作为内存地址来使用，高级编程语言中通常对其做了区分。<br/>
//! 比如你不能拿0x00001111告诉编译器它是一个内存地址，因为编译器可以认为它是一个整数。<br/>
//! 所以我们引入了一个新的内存地址类型，它的两大描述为：它通常占用和字长一样的字节数，我们按内存地址的方式使用它。<br/>
//! 我们声明0x00001111是一个指针，这时编译器便知道了：“奥，0x00001111表达的是内存地址，可以读可以写”。<br/>
//! 由于内存地址本身就是数字编号，所以内存地址类型和等长的整数类型本质上可以互相转换。大部分不屏蔽指针的编程语言，也不限制这种转换。<br/>
//! - 指针变量：也就是存放指针的变量，这和存放int的变量，存放char的变量没有语义上的区别。通常不需要特别讲指针变量，就像不讲整数变量一样，把它当做一般的变量即可。
//! - XXX型指针：描述的是这个指针所表示的地址中，所存放的数据的类型。
//!
//! ## 示例：
//! ```
//! let x:i32 = 42; //x是一个变量，存储一个i32整数，x是一个i32型变量。
//! let x_ptr = &x; //&x表达变量x的地址，&x是一个指针，&x是一个i32型指针
//!                 //x_ptr是一个变量，存储一个(i32型)指针，x_ptr是一个(i32型)指针型变量。
//!                 //基于变量的使用原则，我们只以值的角度来看即可，即x_ptr是一个i32型指针。
//!                 //在Rust语义中，&x是一个受限制的指针，即Rust引用。真正的指针在Rust中被称为原生指针。它们可以互相转换。
//! ```
//!
//! ## &和*的关系：
//! ```
//! 一般&操作表示取变量的内存地址，所以后面一般都是跟一个变量符号。&的语义在变量维度，不关注变量的值。
//! 一般*操作表示从内存地址中取值，所以后面一般都是跟一个指针类型。*的语义在值的维度，可以*x_ptr，也可以*&x，它们等价。
//! ```
//!
//! ## 智能指针：
//! - 智能指针一种增强型的内存地址类型，除了表达内存地址之外，还额外增加了一些描述信息，以及对资源所有权的管理等。
//! - 所以智能指针类型比指针类型占用的字节要大，增加了一部分对使用方式的描述，所以使用方式在“按内存地址的方式使用它”的基础上变得更加详细了。
//! - String型智能指针：表达String在堆上的内存地址，以及长度信息，所有权的管理等。
//! - String型普通指针：表达此内存地址上的数据是String类型。c语言按\0表示字符串的结束。
//!
//! ## 解指针(解引用):
//! 设a为xxx型指针,规定解指针(解引用)操作为*a（*的语义在值的维度）。其表示获得在这个指针中存储的xxx类型。
//! 国内也有把“这个指针中存储的”称为“这个指针指向的”，这是不严谨的。

use crate::memory_and_pointer_test::memory_operation::MemoryOperation;

/// # 名字、标识符、变量
/// 虽然术语“名字”和“变量”通常指的是同一个事物,我们还是要很小心地使用它们,以便区别编译时刻的名字和名字在运行时刻所指的内存位置。  <br/>
///
/// 标识符(identifier)是一个字符串,通常由字母和数字组成。它用来指向(标记)一个实体,比如一个数据对象、过程、类,或者类型。 <br/>
/// 所有的标识符都是名字,但并不是所有的名字都是标识符。 <br/>
/// 名字也可以是一个表达式。比如名字x.y可以表示x所指的一个结构中的字段y.这里,c和y是标识符,而x.y是一个名字。 <br/>
/// 像x.y这样的复合名字称为受限名字(qualifiecname) <br/>
///
/// 变量指向存储中的某个特定的位置。 <br/>
/// 同一个标识符被多次声明是很常见的事情,每一个这样的声明引入一个新的变量。 <br/>
/// 即使每个标识符只被声明一次,一个递归过程中的局部标证符将在不同的时刻指向不同的存储位置。 <br/>
#[test]
fn name_and_identifier() {
    // 栈上一块内存存放String智能指针，let让标识符x绑定到这块内存上。
    let mut string_owner = String::from("hello");
    println!("old String 在堆中的地址：{:p}", string_owner.as_ptr());// 0x2593b8586d0
    let old_string_ref = &mut string_owner;
    println!("old String 指针地址：{:p}", old_string_ref);// 0x80508ff2b0

    // 重新绑定 和 赋值 有着本质区别。
    // 栈上一块新的内存存放String智能指针，let让标识符x绑定到这块新的内存上。
    let string_owner = String::from("world");
    let new_string_ref = &string_owner;
    println!("new String 指针地址：{:p}", new_string_ref);// 0x80508ff368

    // 无效的找回所有权方法
    // let old_string_owner = *old_string_ref;// cannot move out

    // 找回所有权
    let old_string_owner = std::mem::replace(old_string_ref, String::new());
    println!("old String 在堆中的地址：{:p}", old_string_owner.as_ptr());// 0x2593b8586d0 堆中地址一致，资源被找回
    println!("old String 指针地址：{:p}", &old_string_owner);// 0x80508ff3d0


    println!("旧字符串所有者： 不存在可用的所有者");
    println!("旧字符串引用：{}", old_string_ref);// 空
    println!("新字符串所有者：{}", string_owner);// world
    println!("新字符串引用：{}", new_string_ref);// world
}


/// # rust引用与原生指针
/// 引用是Rust中的安全指针，原生指针是Rust中的不安全指针
/// 原生指针需在unsafe块内使用，具有极大破坏性，突破了所有权限制，突破了可变性限制。
#[test]
fn ref_ptf_test() {
    //给x赋值10 变量标识x本身代表一个栈上的内存地址，此操作是为此地址赋值。
    let mut x = 10;
    //取x的引用（安全指针）
    let x_ref = &x;
    println!("打印指针 引用类型: {:p}", x_ref);//{:p} 将指针类型按16进制形式打印
    //取x的可变引用（安全指针），同时x_ptr失效
    let mut x_mut_ref = &mut x;
    println!("打印指针 引用类型: {:p}", x_mut_ref);
    //安全指针转为原生指针 *const表示原生不可变指针， *mut表示原生可变指针。
    let x_mut_raw_ptr = x_mut_ref as *mut i32;
    println!("打印指针 原生指针类型: {:p}", x_mut_raw_ptr);
    //原生指针转为整数型
    let x_addr_int = x_mut_raw_ptr as usize;
    println!("打印指针 整数类型: 0x{:X}", x_addr_int);//{:X} 将整形按16进制格式打印。
    /*打印结果：
            打印指针 引用类型: 0x68ff9ff174
            打印指针 引用类型: 0x68ff9ff174
            打印指针 原生指针类型: 0x68ff9ff174
            打印指针 整数类型: 0x68FF9FF174
      可见地址都是一样的，只是对这些类型使用的限制各有不同。
    */
}


/// # 栈上内存测试
/// 局部变量都在栈内存中。
#[test]
fn memory_stack_test() {
    let x = 10;// &x 取x的变量地址（栈内存地址）

    //打印内存信息
    MemoryOperation::print_addr_and_value(&x as *const i32 as usize, 4);
    /*打印结果：栈上内存直接存放字面量值
        addr:0x68FF9FF174 value:0xA
        addr:0x68FF9FF175 value:0x0
        addr:0x68FF9FF176 value:0x0
        addr:0x68FF9FF177 value:0x0     //即0x0000000A 即整数10
    */
}


/// # 堆上内存测试
///
/// ## 堆内存 与 变量
/// ```
/// // 字面量"test"存储在静态存储区，随着二进制文件的生成由编译器分配，在程序的整个运行期都会存在。
/// // 所以字面量不给出所有权，不能修改，只能引用：&str。
/// // to_string() 函数将"test"复制并放在堆中，返回一个String类型智能指针，包含堆上地址，长度等。
/// "test".to_string()
/// ```
///
/// rust中的Box、Arc、以及String等类型本质上都是智能指针。
/// 与c语言不同, c语言没有那么多类型, 所以c语言实现很多复杂类型需要自行设计. 这一点rust通过智能指针预先设计好了很多类型.
///
/// ## 强制获取堆上数据的内存地址
/// - 方式1：强转法  &x->原生指针->整数型地址->usize类型原生指针->取出usize 整数型地址为0x0000029ACC686810
/// - 方式2：as_ptr() 返回堆上数据的u8型原生指针 指针的值为0x0000029ACC686810
#[test]
fn memory_heap() {
    // 将智能指针String绑定到x上。
    let mut x = "test".to_string();
    println!("x 变量地址: {:p}", &x);// 0xfa4defe3b0
    println!("x 变量值: {}", x);//打印x println!会自动处理解引用
    // println!("x 变量值: {}", *x); //报错：doesn't have a size known at compile-time。 在编译时不知道size。*x，即[[0xfa4defe3b0]]，即[0x0000029ACC686810]，
    //                             //此处是't'的存放位置，又没有size约束，不像c语言以'\0'做字符串结尾。所以此处在编译时不知道size。
    // println!("x 变量值: {:p}", x);//报错：the trait `std::fmt::Pointer` is not implemented for `std::string::String`。String是Rust预置的智能指针类型，其没有实现Pointer特性。
    //                             //不能按普通指针的形式打印。不像c中的String需要自己用指针实现。c中一般只有基础类型，复杂类型都要通过指针来设计实现。


    println!("\r\n-------String智能指针的内存模型--------");
    let x_addr_int = MemoryOperation::convert_addr_to_int(&x);
    println!("x 变量地址（整数型）: 0x{:X}", x_addr_int);// 0xFA4DEFE3B0
    MemoryOperation::print_addr_and_value(x_addr_int, 16);
    /* String智能指针的内存模型：
     *   addr:0xFA4DEFE3B0 value:0x10
     *   addr:0xFA4DEFE3B1 value:0x68
     *   addr:0xFA4DEFE3B2 value:0x68
     *   addr:0xFA4DEFE3B3 value:0xCC
     *   addr:0xFA4DEFE3B4 value:0x9A
     *   addr:0xFA4DEFE3B5 value:0x2
     *   addr:0xFA4DEFE3B6 value:0x0
     *   addr:0xFA4DEFE3B7 value:0x0  //0x0000029ACC686810 64bit 表示字符串"test"在堆上的地址
     *   addr:0xFA4DEFE3B8 value:0x4
     *   addr:0xFA4DEFE3B9 value:0x0
     *   addr:0xFA4DEFE3Ba value:0x0
     *   addr:0xFA4DEFE3Bb value:0x0
     *   addr:0xFA4DEFE3Bc value:0x0
     *   addr:0xFA4DEFE3Bd value:0x0
     *   addr:0xFA4DEFE3Be value:0x0
     *   addr:0xFA4DEFE3Bf value:0x0  //0x0000000000000004 64bit 表示字符串的长度
    */

    println!("\r\n-------as_ptr() 方法研究--------");
    let x_as_mut_ptr = x.as_mut_ptr();//as_ptr() 返回u8型原生指针，为字符串在堆上的地址
    println!("x_as_mut_ptr 指针值: {:p}", x_as_mut_ptr);// 0x0000029ACC686810
    unsafe {
        println!("x_as_mut_ptr 解引用值（t的ascii编码）: 0x{:X} ", *x_as_mut_ptr);//0x74
        println!("x_as_mut_ptr 解引用值转char类型: {}", *x_as_mut_ptr as char);//t
        let x_u8_ref = &*x_as_mut_ptr;//解出值再引用 转为u8型Rust引用
        println!("x_as_mut_ptr 解引用再取地址: {:p}", x_u8_ref);//还是0x0000029ACC686810
        *x_as_mut_ptr = *x_as_mut_ptr + 1; //改0x74为0x75
    }
    println!("x 变量值: {} （第一个字符被使用原生指针进行了强改）", x); //uest

    println!("\r\n-------move 语义研究--------");
    println!("x_addr_int：0x{:X}", x_addr_int);
    let y = x;//转移所有权 右边是值,左边是一个新位置. Move后由新变量y来存储String智能指针，旧变量x作废。
    let y_addr_int = MemoryOperation::convert_addr_to_int(&y); //取y的地址 整数形式
    println!("y_addr_int：0x{:X}", y_addr_int);//以Hex形式打印
    println!("y_as_ptr 变量值: {:p}", y.as_ptr());//0x0000029ACC686810 String智能指针没变，堆上地址没变
    MemoryOperation::print_addr_and_value(y_addr_int, 16);
}


/// # 地址链式测试
/// 验证操作符 & 为取变量自身地址的功能。
#[test]
fn test_ampersand() {
    let x = String::from("hello");
    let a = &x;// 创建变量a，存放x的地址
    let b = &a;// 创建变量b，存放a的地址
    let c = &b;// 创建变量c，存放b的地址

    MemoryOperation::print_addr_and_value(MemoryOperation::convert_addr_to_int(c), 8);
    MemoryOperation::print_addr_and_value(MemoryOperation::convert_addr_to_int(b), 8);
    MemoryOperation::print_addr_and_value(MemoryOperation::convert_addr_to_int(a), 16);
}
