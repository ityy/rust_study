//! 指针学习研究详解
//! 指针是地址还是变量 带来的迷惑点解答
//! 变量的三个维度：变量类型，变量地址，变量值 带来的迷惑点解答
//! move语义的研究 box指针的研究
//! 各种指针的强转，强取内存中的数据，强取指针变量中的内存地址，修改内存中的数据等。
//!
//! 一个数据在内存中，有地址（address）和值（value）两个属性。
//! ----------
//! | address  |
//! ----------
//! |  value   |
//! ----------
//!
//! 变量的本质：
//!     以汇编为例： mov [0x00001111],1 表示向内存地址0x00001111处写入1。
//!     一般高级编程语言中，我们用变量标识符来替代汇编语言中的 [0x00001111] 符号，以表达某个内存地址，可读可写。
//!     例如一个变量a，通常a表示[0x00001111]，它是一个地址空间，可读可写。&a表示a的地址0x00001111。
//!     一个大原则：我们对一切变量的使用，都是从值的角度来使用的。只有&操作符存在时，我们才关注变量自身的内存地址。
//!     由于变量都是在栈上存放，所以‘&变量’取到的地址通常都是栈上的地址。
//!     严格来讲：对于局部变量，是[ebp+xxx]的xxx 即ebp的偏移地址
//!             对于全局变量，则是标号（）
//! 指针变量：
//!     存放内存地址的变量。明确一个观点：指针就是指针变量的简称，没有任何指向之类的说法或描述，这些说法都是错的。
//!     我们单独把存放内存地址的变量独立出来叫做指针变量，简称指针。它和变量这个抽象名词是同维度的。
//!     变量有类型，指针变量也有类型。指针变量的类型是对指针中保存的内存地址怎么使用的描述。
//!     很多人认为指针就是一个内存地址，这是角度不同引起的迷惑。例如变量a，a保存了一个内存地址，那么：
//!         变量角度：a是一个指针。它拥有变量自身的地址、变量值、变量类型，和普通变量一致。使用&a可以取出变身自身的地址。
//!         值的角度：a是一个内存地址。它是某类型的内存地址，使用*a可以按某类型取出此地址上的值。
//! 智能指针变量：
//!     普通指针变量仅保存了内存地址，智能指针变量保存了额外的描述信息（长度等）。
//! &和*的关系：
//!     一般&操作表示取变量的内存地址，所有后面一般都是跟一个变量。&的语义在变量维度，不关注变量的值。严格以维度区分，可避免错误。
//!     一般*操作表示从内存地址中取值，所以后面一般都是跟一个指针。*的语义在值的维度，与+ -等各类运算符一致。
//! 解指针(解引用):
//!     设a为指针,规定解指针(解引用)操作为*a。其表示获得以指针值为起始内存地址，以指针类型为类型的对象。
//! 总结：
//!     不管是传统变量，还是指针，智能指针：我们关注变量的时候，更多的是关注变量的值和类型，极少关注变量自身的地址。
//!     看到变量，只要没有&取地址操作，那都只需要关注变量值即可。


#[test]
/// 栈上内存学习
fn memory_stack() {
    ///rust引用（指针） 原生指针 整数型地址
    let mut x = 10;//给x赋值10 变量标识x本身代表一个栈上的内存地址，此操作为此地址赋值。
    let mut x_ptr = &mut x;//取x自身的内存地址
    let x_raw_ptr = x_ptr as *mut i32;//将引用转为原生指针  注：原生指针可在unsafe块内使用，具有极大破坏性，突破了所有权限制，突破了可变性限制。
    let x_addr_int = x_raw_ptr as usize;//取x的内存地址（整形形式） 可以通过将原生指针转换为整数的形式获得。
    println!("变量x的地址 引用（指针）类型: {:p}", x_ptr);//编程中使用变量标识，都是使用它所存储的值。x_ptr中存储着变量x的地址。
    println!("变量x的地址 原生指针类型: {:p}", x_raw_ptr);//{:p} 将变量的值按地址格式打印，等于打印指针。
    println!("变量x的地址 整数类型: 0x{:X}", x_addr_int);//{:X} 将整形按16进制格式打印。
    /*打印结果：
        变量x的地址 引用（指针）类型: 0x65ca6ff2ac
        变量x的地址 原生指针类型: 0x65ca6ff2ac
        变量x的地址 整数类型: 0x65CA6FF2AC
      可见都是一样的，只是对这些类型使用的限制各有不同。
    */

    ///打印内存信息
    print_addr_and_value(x_addr_int, 4);
    /*打印结果：栈上内存直接存放着值, 没有堆上内存地址这一中间信息
        addr:0x65CA6FF2AC value:0xA
        addr:0x65CA6FF2AD value:0x0
        addr:0x65CA6FF2AE value:0x0
        addr:0x65CA6FF2AF value:0x0
    */
}

///编译器生成临时地址学习
#[test]
fn temp_memory_addr() {
    //由于42是字面量，是一个只读值。这里编译器会给&42临时创建一个内存地址
    let temp_addr1 = &42;
    let temp_addr2 = &&42;
    let temp_addr3 = &&&42; //3个临时地址几乎是连续分配的。
    println!("temp_addr1 变量地址: {:p}", &temp_addr1);
    println!("temp_addr2 变量地址: {:p}", &temp_addr2);
    println!("temp_addr3 变量地址: {:p}", &temp_addr3);
    println!("temp_addr1 变量值: {:p}", temp_addr1);
    println!("temp_addr2 变量值: {:p}", temp_addr2);
    println!("temp_addr3 变量值: {:p}", temp_addr3);
    /*打印结果
        temp_addr1 变量地址: 0x48c24fee08
        temp_addr2 变量地址: 0x48c24fee10
        temp_addr3 变量地址: 0x48c24fee18
        temp_addr1 变量值: 0x7ff6ee7601e0
        temp_addr2 变量值: 0x7ff6ee7601e8
        temp_addr3 变量值: 0x7ff6ee7601f0
    */
}

/// 堆上内存学习
#[test]
fn memory_heap() {
    /// 堆内存 与 变量
    /// 凡是变量绑定了一个堆上内存的目标对象时，变量均为指针变量，其存放着目标对象在堆上的内存地址。
    // x是一个位置表达式 右边字面量是一个值表达式
    // to_string()函数将"test"放在堆中，并返回一个String类型智能指针。
    // 此时x为一个String类型的智能指针(smart pointer)。
    let mut x = "test".to_string();
    println!("x 变量值: {}", x);//打印x println!会自动处理解引用
    // println!("x 变量值: {}", *x); //报错：doesn't have a size known at compile-time。 在编译时不知道size。*x，即[[0xFA4DEFE3B0]]，即[0x0000029ACC686810]，
    //                             //此处是't'的存放位置，又没有size约束，不像c语言以'\0'做字符串结尾。所以此处在编译时不知道size。
    // println!("x 变量值: {:p}", x);//报错：the trait `std::fmt::Pointer` is not implemented for `std::string::String`。String是Rust预置的智能指针类型，
    //                             //不能按单指针的形式打印。不像c中的String需要自己用指针实现。c中一般只有基础类型，复杂类型都要通过指针来设计实现。

    /// 指针转换
    println!("\r\n-------指针转换--------");
    let x_ptr = &x; //取x的地址放入x_ptr
    println!("x_ptr 变量值: {:p}", x_ptr);//将其值按内存地址格式打印
    println!("x_ptr 解引用值: {}", *x_ptr);//*加不加都可以, println!可以自动解引用来打印值. 只有用{:p}才会打印地址.
    let x_mut_ptr = &mut x; //此处获取了x的可变借用，则之后不能再使用x_ptr（x的不可变借用），否则报错。
    println!("x_mut_ptr 变量值: {:p}", x_mut_ptr);
    println!("x_mut_ptr 解引用值: {}", *x_mut_ptr);
    let x_raw_ptr = x_mut_ptr as *const String;//将引用转换为原生指针。 *const表示原生不可变地址， *mut表示原生可变地址。
    println!("x_raw_ptr 变量值: {:p}", x_raw_ptr);
    //必须在unsafe块内使用原生指针。
    unsafe {
        println!("x_raw_ptr 解引用值: {}", *x_raw_ptr);
    }

    ///打印内存模型
    println!("\r\n-------以&x开始打印16byte的内存数据--------");
    let x_addr_int = convert_addr_to_int(&x);
    println!("x_addr_int 变量值: 0x{:X}", x_addr_int);// 以16进制打印x_addr_int
    //打印结果：
    //  0x000000FA4DEFE3B0  （64位内存地址）
    print_addr_and_value(x_addr_int, 16);
    // 内存模型：
    //   addr:0xFA4DEFE3B0 value:0x10
    //   addr:0xFA4DEFE3B1 value:0x68
    //   addr:0xFA4DEFE3B2 value:0x68
    //   addr:0xFA4DEFE3B3 value:0xCC
    //   addr:0xFA4DEFE3B4 value:0x9A
    //   addr:0xFA4DEFE3B5 value:0x2
    //   addr:0xFA4DEFE3B6 value:0x0
    //   addr:0xFA4DEFE3B7 value:0x0  //0x0000029ACC686810 64bit 表示字符串"test"在堆上的地址
    //   addr:0xFA4DEFE3B8 value:0x4
    //   addr:0xFA4DEFE3B9 value:0x0
    //   addr:0xFA4DEFE3Ba value:0x0
    //   addr:0xFA4DEFE3Bb value:0x0
    //   addr:0xFA4DEFE3Bc value:0x0
    //   addr:0xFA4DEFE3Bd value:0x0
    //   addr:0xFA4DEFE3Be value:0x0
    //   addr:0xFA4DEFE3Bf value:0x0  //0x0000000000000004 64bit 表示字符串的长度
    //
    // rust中的Box,Arc,以及String等类型本质上都是智能指针.
    // 与c语言不同, c语言没有那么多类型, 所以c语言实现很多复杂类型需要自行设计. 这一点rust通过智能指针预先设计好了很多类型.

    ///强制获取堆上数据的内存地址
    /// 方式1：强转法 可以将&x强转为当前类型原生指针->整数型地址->usize类型原生指针->取出usize 此为堆上数据的整数型地址0x0000029ACC686810
    /// 方式2：as_ptr() 返回堆上数据的u8型原生指针 指针的值为0x0000029ACC686810
    println!("\r\n-------as_ptr() 方法研究--------");
    let x_as_mut_ptr = x.as_mut_ptr();//as_ptr()返回u8型原生指针 值为堆上的地址0x0000029ACC686810
    println!("x_as_mut_ptr 变量值: {:p}", x_as_mut_ptr);//0x0000029ACC686810
    unsafe {
        println!("x_as_mut_ptr 解引用值: 0x{:X} （t的ascii编码）", *x_as_mut_ptr);
        println!("x_as_mut_ptr 解引用值(ascii字符): {}", *x_as_mut_ptr as char);//0x74
        println!("x_as_mut_ptr 解引用再取地址: {:p}", &*x_as_mut_ptr);//解出值，再取此值的地址，还是0x0000029ACC686810 不会生成临时地址
        *x_as_mut_ptr = *x_as_mut_ptr + 1; //改0x74为0x75
    }
    println!("x 变量值: {} （第一个字符被使用原生指针进行了强改）", x); //uest

    ///move语义研究
    println!("\r\n-------Move--------");
    println!("x_addr_int：0x{:X}", x_addr_int);
    let y = x;//转移所有权 右边是值,左边是一个新位置. Move后由新指针y来存储字符串的地址0x0000029ACC686810和大小0x0000000000000004，旧指针x作废。
    let y_addr_int = convert_addr_to_int(&y); //取y的地址 整数形式
    println!("y_addr_int：0x{:X}", y_addr_int);//以Hex形式打印
    println!("y_as_ptr 变量值: {:p}", y.as_ptr());//0x0000029ACC686810
}

///Box指针·研究
#[test]
fn test_box() {
    println!("\r\n-------Box智能指针解析--------");
    let box_smart_ptr = Box::new("test".to_string());//再次转移所有权 交给指针Box 旧指针y作废
    println!("box_smart_ptr 变量值: {}", box_smart_ptr);
    println!("box_smart_ptr 变量值(p格式): {:p}", box_smart_ptr);//Box类型可以直接打印指针地址
    println!("box_smart_ptr 解引用值: {}", *box_smart_ptr);
    let content_ptr = box_smart_ptr.as_ptr();
    println!("content_ptr 变量值(p格式): {:p}", content_ptr);//移动后堆上的内存地址是不变的 仍为0x0000029ACC686810

    println!("\r\n-------Box智能指针 内存结构--------");
    let box_addr_int = convert_addr_to_int(&box_smart_ptr);
    println!("box_addr_int 变量值: 0x{:X}", box_addr_int);// 以16进制打印z_addr_int 64位的内存地址：0x000000FA4DEFE3B0
    print_addr_and_value(box_addr_int, 24);
    /*打印结果：
        共24byte，前8byte为String智能指针的地址，此指针被挪到了堆上。后16byte为content在堆上的地址及长度。
    */
    println!("\r\n-------以box_value_int开始打印16byte的内存数据--------");
    unsafe {
        /// 强取指针变量中保存的指针的方法：
        /// 1 取指针变量的地址，并强转为整数型地址
        /// 2 将整数型地址 强转为 usize型指针
        /// 3 即可使用解引用的方式获取一个usize型的整数值
        let box_usize_ptr = box_addr_int as *mut usize;
        let box_value_int = *box_usize_ptr;
        println!("box_value_int 变量值: 0x{:X}", box_value_int);// 以16进制打印z_int 0x000000FA 4DEFE3B0 64位的内存地址
        print_addr_and_value(box_value_int, 16);
    }
    /// 整数型地址 转回Rust引用
    let temp = unsafe { convert_int_to_addr::<Box<String>>(box_addr_int) };
    println!("整数型地址，转回Rust引用：{}", temp);
    /*打印结果：
        共16byte，为String在堆上的地址及长度。
    */

    /*总结：
        增加了Box指针后，原String指针被挪到了堆上，Box指针指向堆上的String指针，String指针再指向堆上的字符串内容。
        等于说是在中间加了一层指针包装。
    */
}


///多重Box
#[test]
fn testBox2() {
    let x = Box::new("hello".to_string());
    let y = Box::new(x);
    let z = Box::new(y);
    println!("{}", ***z);
    let z_addr_int = convert_addr_to_int(&z);
    print_addr_and_value(z_addr_int, 24);
}

///将内存地址(Rust引用型)转为整数类型
pub fn convert_addr_to_int<T>(t: &T) -> usize {
    //rust引用和raw指针 raw指针和usize 可以互相转换, 这就为内存操作提供了无限可能
    //*const表示原生不可变指针， *mut表示原生可变指针。
    t as *const T as usize
}

///将整数类型转为内存地址(Rust引用型)
pub unsafe fn convert_int_to_addr<'a, T>(addr: usize) -> &'a T {
    let raw_ptr = addr as *const T;//1 将整数型地址转为原生指针,并指定类型
    &*raw_ptr//2 解引用原生指针得到具体类型的对象,再取地址(即Rust指针,即Rust引用)
}


///打印内存地址
pub fn print_addr_and_value(addr_begin: usize, byte_count: usize) {
    //需要访问原生指针，则必须在unsafe块中使用
    unsafe {
        for i in 0..byte_count {
            let x = (addr_begin + i) as *const u8;//将整数型地址，转回raw指针。此处要打印字节，所以按u8类型指针处理。
            println!("addr:0x{:X} value:0x{:X}", addr_begin + i, *x); //读地址处的值非常方便，使用*操作符即可。这两句理论上可以获取任意内存地址上的值。
        }
    }
}