//! 变量、指针等 学习研究详解
//! 变量的三个维度：变量类型，变量地址，变量值 带来的迷惑点解答
//! move语义的研究 box指针的研究
//! 各种指针的强转，强取指针中的数据，强取指针变量中的内存地址（作为整数），修改整数型内存地址中的数据等。
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
//!             一般高级编程语言中，我们用变量标识符来替代汇编语言中的 [0x00001111] 符号，以表达某个内存地址，可读可写。
//!             例如一个变量a，通常a表示[0x00001111]，它是一个地址空间，可读可写。
//!             严格来讲，对于局部变量，是[ebp+xxx]的xxx 即ebp的偏移地址，对于全局变量，则是标号（）。
//!     变量地址：&操作符可以用来获取变量的地址，&a表达的便是a的地址0x00001111。由于变量都是在栈上存放，所以‘&变量’取到的地址一般都是栈上的地址。
//!     变量的使用原则：我们对一切变量的使用，都是从值的角度来使用的。只有&操作符存在时，我们才关注变量自身的内存地址。
//!     数据类型：是对一段数据的两大描述：占了多少个字节，以什么方式来使用它。
//!             编程语言预置了一些数据类型，我们也可以自定义数据类型。我们在计算机的历史上不断的增加各种使用方式。
//!                 int型：   4字节                 我们按数值的方式使用它
//!                 char型：  1字节                 我们按符号的方式使用它
//!                 指针型：   64字节（64bit主机）    我们按内存地址的方式使用它   由于指针型权限太大，一些编程语言屏蔽了它，只开放了引用型，即受限制的指针。
//!                 ...
//!     变量类型：即变量中存储的数据的数据类型。强类型语言中，某类型的变量只能存放某类型的数据。
//! 指针的本质：
//!     指针：指针就是一种数据类型，表达的是内存地址类型。不像汇编中可以直接拿一个数值比如0x00001111作为内存地址来使用，高级编程语言中通常对其做了区分。
//!         比如你不能拿0x00001111告诉编译器它是一个内存地址，因为编译器可以认为它是一个整数。
//!         所以我们引入了一个新的内存地址类型，它的两大描述为：它通常占用和字长一样的字节数，我们按内存地址的方式使用它。
//!         我们声明0x00001111是一个指针，这时编译器便知道了：“奥，0x00001111表达的是内存地址，可以读可以写”。
//!         由于内存地址本身就是数字编号，所以内存地址类型和等长的整数类型本质上可以互相转换。大部分不屏蔽指针的编程语言，也不限制这种转换。
//!     指针变量：也就是存放指针的变量，这和存放int的变量，存放char的变量没有语义上的区别。通常不需要特别讲指针变量，就像不讲整数变量一样，把它当做一般的变量即可。
//!     XXX型指针：描述的是这个指针所表示的地址中，所存放的数据的类型。
//!     举例：
//!     ```
//!     let x:i32 = 42; //x是一个变量，存储一个i32整数，x是一个i32型变量。
//!     let x_ptr = &x; //&x表达变量x的地址，&x是一个指针，&x是一个i32型指针
//!                     //变量角度，x_ptr是一个变量，存储一个(i32型)指针，x_ptr是一个(i32型)指针型变量。
//!                     //基于变量的使用原则，我们只以值的角度来看即可，即x_ptr是一个i32型指针。
//!                     //在Rust语义中，&x是一个受限制的指针，即Rust引用。真正的指针在Rust中被称为原生指针。好在它们可以互相转换。
//!     ```
//! &和*的关系：
//!     一般&操作表示取变量的内存地址，所有后面一般都是跟一个变量符号。&的语义在变量维度，不关注变量的值。
//!     一般*操作表示从内存地址中取值，所以后面一般都是跟一个指针类型。*的语义在值的维度，可以*x_ptr，也可以*&x，它们等价。
//! 智能指针：
//!     智能指针一种增强型的内存地址类型，除了表达内存地址之外，还额外增加了一些描述信息，以及对资源所有权的管理等。
//!     所以智能指针类型比指针类型占用的字节要大，增加了一部分对使用方式的描述，所以使用方式在“按内存地址的方式使用它”的基础上变得更加详细了。
//!     String型智能指针：表达String在堆上的内存地址，以及长度信息，所有权的管理等。
//!     String型普通指针：表达此内存地址上的数据是String类型。c语言按\0表示字符串的结束。
//! 解指针(解引用):
//!     设a为xxx型指针,规定解指针(解引用)操作为*a。其表示获得在这个指针中存储的xxx类型。（国内也有把“这个指针中存储的”称为“这个指针指向的”，这是不严谨的）


#[test]
/// 栈上内存学习
fn memory_stack() {
    /// rust引用 原生指针
    /// 引用是Rust中的安全指针，原生指针是Rust中的不安全指针
    /// 原生指针需在unsafe块内使用，具有极大破坏性，突破了所有权限制，突破了可变性限制。
    let mut x = 10;//给x赋值10 变量标识x本身代表一个栈上的内存地址，此操作是为此地址赋值。
    let mut x_ptr = &mut x;//取x的可变引用（安全指针）
    let x_raw_ptr = x_ptr as *mut i32;//安全指针转为原生指针
    let x_addr_int = x_raw_ptr as usize;//原生指针转为整数型
    println!("打印指针 引用类型: {:p}", x_ptr);//{:p} 打印指针，即将指针类型按16进制形式打印。
    println!("打印指针 引用类型: {:p}", &x);// x_ptr与&x等价，都是指针
    println!("打印指针 原生指针类型: {:p}", x_raw_ptr);
    println!("打印指针 整数类型: 0x{:X}", x_addr_int);//{:X} 将整形按16进制格式打印。
    /*打印结果：
            打印指针 引用类型: 0x68ff9ff174
            打印指针 引用类型: 0x68ff9ff174
            打印指针 原生指针类型: 0x68ff9ff174
            打印指针 整数类型: 0x68FF9FF174
      可见地址都是一样的，只是对这些类型使用的限制各有不同。
    */

    ///打印内存信息
    print_addr_and_value(x_addr_int, 4);
    /*打印结果：栈上内存直接存放着值
        addr:0x68FF9FF174 value:0xA
        addr:0x68FF9FF175 value:0x0
        addr:0x68FF9FF176 value:0x0
        addr:0x68FF9FF177 value:0x0     //即0x0000000A 即整数10
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
    // to_string()函数将"test"放在堆中，并返回一个String类型智能指针，包含堆上地址，长度等。
    // 将智能指针绑定到x上。
    let mut x = "test".to_string();
    println!("x 变量值: {}", x);//打印x println!会自动处理解引用
    // println!("x 变量值: {}", *x); //报错：doesn't have a size known at compile-time。 在编译时不知道size。*x，即[[0xFA4DEFE3B0]]，即[0x0000029ACC686810]，
    //                             //此处是't'的存放位置，又没有size约束，不像c语言以'\0'做字符串结尾。所以此处在编译时不知道size。
    // println!("x 变量值: {:p}", x);//报错：the trait `std::fmt::Pointer` is not implemented for `std::string::String`。String是Rust预置的智能指针类型，
    //                             //不能按普通指针的形式打印。不像c中的String需要自己用指针实现。c中一般只有基础类型，复杂类型都要通过指针来设计实现。

    /// 取变量x的地址作为指针 进行转换测试
    println!("\r\n-------指针转换--------");
    let x_ptr = &x; //取x的引用（安全指针）
    println!("x_ptr 指针值: {:p}", x_ptr);//打印指针
    println!("x_ptr 解引用值: {}", *x_ptr);//*加不加都可以, println!可以自动解引用来打印值. 只有用{:p}才会打印地址.
    let x_mut_ptr = &mut x; //此处获取了x的可变借用，则之后不能再使用x_ptr（x的不可变借用），否则报错。
    println!("x_mut_ptr 指针值: {:p}", x_mut_ptr);
    println!("x_mut_ptr 解引用值: {}", *x_mut_ptr);
    let x_raw_ptr = x_mut_ptr as *const String;//将引用转换为原生指针。 *const表示原生不可变地址， *mut表示原生可变地址。
    println!("x_raw_ptr 指针值: {:p}", x_raw_ptr);
    unsafe {
        //原生指针必须在unsafe块内使用。
        println!("x_raw_ptr 解引用值: {}", *x_raw_ptr);
    }

    /// 取变量x的地址作为起始地址 打印若干字节的内存信息
    println!("\r\n-------打印String智能指针的内存模型--------");
    let x_addr_int = convert_addr_to_int(&x);
    println!("x_addr_int 整数值: 0x{:X}", x_addr_int);// 以16进制打印x_addr_int
    //打印结果：
    //  0x000000FA4DEFE3B0  （64位内存地址）
    print_addr_and_value(x_addr_int, 16);
    // String智能指针的内存模型：
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
    println!("x_as_mut_ptr 指针值: {:p}", x_as_mut_ptr);//0x0000029ACC686810
    unsafe {
        println!("x_as_mut_ptr 解引用值: 0x{:X} （t的ascii编码）", *x_as_mut_ptr);
        println!("x_as_mut_ptr 解引用值(ascii字符): {}", *x_as_mut_ptr as char);//0x74
        let x_u8_ref = &*x_as_mut_ptr;//解出值再引用，可以转为u8型Rust引用
        println!("x_as_mut_ptr 解引用再取地址: {:p}", x_u8_ref);//还是0x0000029ACC686810
        *x_as_mut_ptr = *x_as_mut_ptr + 1; //改0x74为0x75
    }
    println!("x 变量值: {} （第一个字符被使用原生指针进行了强改）", x); //uest

    ///move语义研究
    println!("\r\n-------move语义研究--------");
    println!("x_addr_int：0x{:X}", x_addr_int);
    let y = x;//转移所有权 右边是值,左边是一个新位置. Move后由新变量y来存储String智能指针，旧变量x作废。
    let y_addr_int = convert_addr_to_int(&y); //取y的地址 整数形式
    println!("y_addr_int：0x{:X}", y_addr_int);//以Hex形式打印
    println!("y_as_ptr 变量值: {:p}", y.as_ptr());//0x0000029ACC686810 String智能指针没变，堆上地址没变
}

///Box指针·研究
#[test]
fn test_box() {
    println!("\r\n-------Box智能指针解析--------");
    let box_smart_ptr = Box::new("test".to_string());//再次转移所有权给智能指针Box，并返回一个智能指针Box放入变量box_smart_ptr中。旧变量y因失去String的所有权而作废。
    println!("box_smart_ptr 变量值: {}", box_smart_ptr); //Box类型在使用时可以自动解引用
    println!("box_smart_ptr 指针值: {:p}", box_smart_ptr);//Box类型可以直接打印指针地址 0x209a3c33d90 即String智能指针的所在地址
    println!("box_smart_ptr 解引用值: {}", *box_smart_ptr);
    let content_ptr1 = box_smart_ptr.as_ptr();//相当于(*box_smart_ptr).as_ptr() Box类型在使用时可以自动解引用
    let content_ptr2 = (*box_smart_ptr).as_ptr();
    println!("content_ptr1 指针值: {:p}", content_ptr1);//移动后堆上的内存地址是不变的 仍为0x0000029ACC686810
    println!("content_ptr2 指针值: {:p}", content_ptr2);//移动后堆上的内存地址是不变的 仍为0x0000029ACC686810

    println!("\r\n-------Box智能指针 内存结构--------");
    let box_addr_int = convert_addr_to_int(&box_smart_ptr);
    println!("变量box_smart_ptr的内存地址: 0x{:X}", box_addr_int);// 0x866AAFF038
    println!("智能指针Box的内存模型：");
    print_addr_and_value(box_addr_int, 24);
    /*打印结果：
        共24byte，前8byte为String智能指针的所在地址0x209a3c33d90，此智能指针被挪到了堆上。后16byte为content在堆上的地址及长度。
    */
    /// 整数型地址 转回Rust引用
    let temp = unsafe { convert_int_to_addr::<Box<String>>(box_addr_int) };
    println!("整数型地址，转回Rust引用：{}", temp);

    println!("\r\n-------打印String智能指针的内存模型--------");
    unsafe {
        /// 强取指针变量中保存的指针为整数的方法：
        /// 1 取变量的地址，并强转为原生指针，再将原生指针强转为整数型
        let box_usize_ptr_int = &box_smart_ptr as *const Box<String> as usize;
        /// 2 将整数型 强转为 usize型原生指针
        let box_usize_ptr = box_usize_ptr_int as *const usize;
        /// 3 使用*取出指针中的usize型的整数值
        let box_usize = *box_usize_ptr;
        println!("box_usize 变量值: 0x{:X}", box_usize);//String智能指针的所在地址0x209A3C33D90
        print_addr_and_value(box_usize, 16);
        /*打印结果：
            共16byte，为String在堆上的地址及长度。
        */
        /// 返璞归真 利用提取的String智能指针所在的内存地址 导出String智能指针
        let x = convert_int_to_addr::<String>(box_usize);
        println!("x 变量值：{}", x);//返璞归真，回归最开始的地方。这里将通过Box强制获取的String智能指针又给导出了。
    }


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

///将T类型的Rust引用型（安全指针类型，即安全内存地址类型）转为整数类型
pub fn convert_addr_to_int<T>(t: &T) -> usize {
    //rust引用和raw指针 raw指针和usize 可以互相转换, 这就为内存操作提供了无限可能
    //*const表示原生不可变指针， *mut表示原生可变指针。
    t as *const T as usize
}

///将整数类型转为T类型的Rust引用型（安全指针类型，即安全内存地址类型）
pub unsafe fn convert_int_to_addr<'a, T>(addr: usize) -> &'a T {
    let raw_ptr = addr as *const T;//1 将整数型地址转为T型原生指针
    &*raw_ptr//2 原生指针转为Rust引用的方法：解指针得到对象，再引用
}


///打印内存地址
pub fn print_addr_and_value(addr_begin: usize, byte_count: usize) {
    //需要访问原生指针，则必须在unsafe块中使用
    unsafe {
        for i in 0..byte_count {
            let x = (addr_begin + i) as *const u8;//将整数型地址，转为u8型raw指针（此处要打印字节，所以需要u8型指针）。
            println!("addr:0x{:X} value:0x{:X}", addr_begin + i, *x); //取指针x存储的数据
        }
    }
}

