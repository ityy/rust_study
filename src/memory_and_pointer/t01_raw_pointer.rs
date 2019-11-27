//! 指针学习研究详解
//! & * 操作符的本质研究
//! move语义的研究 box指针的研究
//! 各种指针的强转，强取内存中的数据，强取指针变量中的指针，修改内存中的数据等。
//!
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
//!     例如一个变量a，通常a表示[0x00001111]，它是一个空间，可读可写。&a表示a的地址0x00001111。
//!     一般有且只有 &a 语义是关于变量自身的地址的，其它任何时候a都只表示变量的值。
//! 指针变量：
//!     由于变量都是在栈上存放，所以取到的地址都是栈上的地址。
//!     变量存放字面值或者堆上的地址及其描述信息。如果变量存放的是一个内存地址，可以称为指针变量。
//! &和*的关系：
//!     例如一个变量a，假如a保存了一个内存地址，那么变量a就属于指针变量。a的值就是一个指针（内存地址）。
//!     *a语义表示直接从a（的值）这个指针（内存地址）中取值，即[[0x00001111]]，可读可写。
//!     一般&操作表示取内存地址，所有后面一般都是跟一个变量。
//!     一般*操作表示从地址取值，所以后面一般都是跟一个指针。


#[test]
/// 栈上内存学习
fn place_expression_stack() {
    ///safe地址类型 原生unsafe地址类型 整数型地址
    let mut x = 10;//给x赋值10 变量标识x本身代表一个栈上的内存地址，此操作为此地址赋值。
    let mut x_addr = &mut x;//取x所代表的内存地址
    let x_raw_addr = x_addr as *mut i32;//取x的原生地址 将引用地址转为原生地址类型
    let x_addr_value = x_raw_addr as usize;//取x的地址作为普通的整形数值 可以通过将原生地址转换为整数的形式获得。x_addr_value具有极大破坏性，突破了所有权限制，突破了可变性限制。
    println!("变量x的地址 引用类型: {:p}", x_addr);//编程中使用变量标识，都是使用它所存储的值。x_addr中存储着变量x的地址。
    println!("变量x的地址 原生地址类型: {:p}", x_raw_addr);//{:p} 将变量的值按地址格式打印，等于打印指针。
    println!("变量x的地址 整数类型: 0x{:X}", x_addr_value);//{:X} 将整形按16进制格式打印。
    /*打印结果：
        变量x的地址 引用类型: 0x2d633ff154
        变量x的地址 原生地址类型: 0x2d633ff154
        变量x的地址 整数类型: 0x2D633FF154
      可见都是一样的，只是对这些类型使用的限制各有不同。
    */

    ///使用unsafe块 从任意整数型地址取值
    unsafe {
        //从x_addr_value开始, 打印4个字节的内存
        for i in 0..4 {
            let x = (x_addr_value + i) as *mut u8;//将整数型地址 转回原生地址类型 此处要打印字节，所以按u8类型处理
            println!("addr:0x{:X} value:0x{:X}", x_addr_value + i, *x);
        }
    }
    /*打印结果：栈上内存直接存放着值, 没有堆上内存地址这一中间信息
        addr:0x2D633FF154 value:0xA
        addr:0x2D633FF155 value:0x0
        addr:0x2D633FF156 value:0x0
        addr:0x2D633FF157 value:0x0
    */

    ///临时地址学习
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
fn place_expression_heap() {
    ///堆内存 与 变量
    /// 凡是变量绑定了一个堆上内存的目标对象时，变量均为指针，其存放着目标对象在堆上的内存地址。
    let mut x = "test".to_string(); //x是一个位置表达式 右边字面量是一个值表达式 "test".to_string()函数将其放在堆中，并返回一个String类型指针。此时x称为一个String类型的智能指针。
    println!("x 变量值: {}", x);//打印x的值 println!会按指针描述打印出字符串

    ///指针转换
    println!("\r\n-------指针转换--------");
    let x_addr = &x; //取x的地址    之后的新变量都不再打印变量地址，因为它都是一个新的地址，没有意义。
    println!("x_addr 变量值: {:p}", x_addr);//将其值按内存地址格式打印
    println!("x_addr 解引用值: {}", *x_addr);//*加不加都可以, println!可以自动解引用来打印值. 只有用{:p}才会打印地址.
    let x_mut_addr = &mut x; //此处获取了x的可变借用，则之后不能再使用x_addr（x的不可变借用），否则报错。
    println!("x_mut_addr 变量值: {:p}", x_mut_addr);
    println!("x_mut_addr 解引用值: {}", *x_mut_addr);
    let x_raw_addr = x_mut_addr as *const String;//将x的地址转换为原生地址。 *const表示原生不可变地址， *mut表示原生可变地址。
    println!("x_raw_addr 变量值: {:p}", x_raw_addr);
    unsafe {//原生地址少了很多约束，不符合safe原则，必须在unsafe块内执行。
        println!("x_raw_addr 解引用值: {}", *x_raw_addr);
    }

    ///打印内存模型
    println!("\r\n-------以&x开始打印16byte的内存数据--------");
    let x_raw_addr_int = x_raw_addr as usize; //原始指针和usize可以互相转换, 这就为内存操作提供了无限可能
    println!("x_raw_addr_int 变量值: 0x{:X}", x_raw_addr_int);// 以16进制打印x_raw_addr_int 0x000000FA 4DEFE3B0 64位的内存地址
    /// 强制打印内存的方法
    /// 利用 x_raw_addr_int 强制打印内存地址与值 彻底看清内存结构.
    /// 将整数型地址 强转为 u8型指针
    /// 即可用解引用的方式取得存储的u8型的数据了
    unsafe {
        for i in 0..16 {
            let x = (x_raw_addr_int + i) as *mut u8;
            println!("addr:0x{:X} value:0x{:X}", x_raw_addr_int + i, *x);
        }
    }
    // println!("变量x的值:{}", *x); //报错：doesn't have a size known at compile-time。 在编译时不知道size。*x，即[[0xFA4DEFE3B0]]，即[0x0000029ACC686810]，
    //                             //此处是't'的存放位置，又没有size约束，不像c语言以'\0'做字符串结尾。所以此处在编译时不知道size。
    // println!("x 变量值: {:p}", x);//报错：the trait `std::fmt::Pointer` is not implemented for `std::string::String`。String是Rust预置的智能指针类型，
    //                             //不能按单指针的形式打印。不像c中的String需要自己用指针实现。c中一般只有基础类型，复杂类型都要通过指针来设计实现。
    // 内存模型：
    //   x在栈上, &x为0xFA4DEFE3B0, 值为字符串在堆上的起始地址，长度等。所以x是一个字符串类型的智能指针。
    //   以&x开始打印16byte：
    //   addr:0xFA4DEFE3B0 value:0x10
    //   addr:0xFA4DEFE3B1 value:0x68
    //   addr:0xFA4DEFE3B2 value:0x68
    //   addr:0xFA4DEFE3B3 value:0xCC
    //   addr:0xFA4DEFE3B4 value:0x9A
    //   addr:0xFA4DEFE3B5 value:0x2
    //   addr:0xFA4DEFE3B6 value:0x0
    //   addr:0xFA4DEFE3B7 value:0x0  //0x0000029A CC686810 64bit 表示字符串"test"在堆上的地址
    //   addr:0xFA4DEFE3B8 value:0x4
    //   addr:0xFA4DEFE3B9 value:0x0
    //   addr:0xFA4DEFE3Ba value:0x0
    //   addr:0xFA4DEFE3Bb value:0x0
    //   addr:0xFA4DEFE3Bc value:0x0
    //   addr:0xFA4DEFE3Bd value:0x0
    //   addr:0xFA4DEFE3Be value:0x0
    //   addr:0xFA4DEFE3Bf value:0x0  //0x00000000 00000004 64bit 表示字符串的长度
    //
    // rust中的Box,Arc,以及String等类型本质上都是智能指针.
    // 与c语言不同, c语言没有那么多类型, 所以c语言实现很多复杂类型需要自行设计. 这一点rust通过智能指针预先设计好了很多类型.

    ///as_ptr() 方法研究 以及使用指针强改数据
    println!("\r\n-------as_ptr() 方法研究--------");
    let x_as_ptr = x.as_mut_ptr();//as_ptr()获取的直接是堆上的内存地址0x0000029ACC686810
    println!("x_as_ptr 变量值: {:p}", x_as_ptr);//0x0000029ACC686810
    unsafe {
        let ptr_value = *x_as_ptr;//此u8为0x74 即字母t的ascii码  u8是copy型的,ptr_value为新变量,所以这里不是移动 是拷贝
        println!("x_as_ptr 解引用值: {}", *x_as_ptr as char);//0x74
        println!("x_as_ptr 解引用再取地址: {:p}", &*x_as_ptr);//0x0000029ACC686810 此处没有生成临时地址
        println!("ptr_value 变量值: 0x{:X} （这是t的ascii编码,已取到最终地址）", ptr_value);
        *x_as_ptr = *x_as_ptr + 1; //改0x74为0x75
    }
    println!("x 变量值: {} （第一个字符被使用指针进行了强改）", x); //uest

    ///move语义研究
    println!("\r\n-------Move--------");
    println!("x_raw_addr_int：0x{:X}", x_raw_addr_int);
    let y = x;//转移所有权 右边是值,左边是一个新位置. Move后由新指针y来存储字符串的地址0x0000029ACC686810和大小0x0000000000000004，旧指针x作废。
    let y_raw_addr_int = &y as *const String as usize; //取y的地址 整数形式
    println!("y_raw_addr_int：0x{:X}", y_raw_addr_int);//以Hex形式打印
    println!("y_as_ptr 变量值: {:p}", y.as_ptr());//0x0000029ACC686810
}

///Box指针·研究
#[test]
fn test_box() {
    println!("\r\n-------Box--------");
    let z = Box::new("test".to_string());//再次转移所有权 交给指针Box 旧指针y作废
    println!("z 变量值: {}", z);
    println!("z 变量值(p格式): {:p}", z);//Box类型可以直接打印指针地址
    println!("z 解引用值: {}", *z);
    let z_as_ptr = z.as_ptr();
    println!("z_as_ptr 变量值(p格式): {:p}", z_as_ptr);//移动后堆上的内存地址是不变的 仍为0x0000029ACC686810

    println!("\r\n-------以&z开始打印24byte的内存数据(即打印Box的内容)--------");
    let z_addr_int = &z as *const Box<String> as usize; //raw指针和usize可以互相转换, 这就为内存操作提供了无限可能
    println!("z_addr_int 变量值: 0x{:X}", z_addr_int);// 以16进制打印z_addr_int 64位的内存地址：0x000000FA4DEFE3B0
    //利用z_int强制打印内存地址与值 彻底看清内存结构.
    unsafe {
        for i in 0..24 {
            let x = (z_addr_int + i) as *mut u8;
            println!("addr:0x{:X} value:0x{:X}", z_addr_int + i, *x);
        }
    }
    /*打印结果：
        共24byte，前8byte为String指针，此指针被挪到了堆上。后16byte为String在堆上的地址及长度。
    */
    println!("\r\n-------以z_value_int开始打印16byte的内存数据--------");
    unsafe {
        /// 强取指针变量中保存的指针的方法：
        /// 1 取指针变量的地址，并强转为整数型地址
        /// 2 将整数型地址 强转为 usize型指针
        /// 3 即可使用解引用的方式获取一个usize型的整数值
        let z_value_int_ptr = z_addr_int as *mut usize;
        let z_value_int = *z_value_int_ptr;
        println!("z_value_int 变量值: 0x{:X}", z_value_int);// 以16进制打印z_int 0x000000FA 4DEFE3B0 64位的内存地址
        //利用z_int强制打印内存地址与值 彻底看清内存结构.
        for i in 0..16 {
            let x = (z_value_int + i) as *mut u8;
            println!("addr:0x{:X} value:0x{:X}", z_value_int + i, *x);
        }
    }
    /*打印结果：
        共16byte，为String在堆上的地址及长度。
    */

    /*总结：
        增加了Box指针后，原String指针被挪到了堆上，Box指针指向堆上的String指针，String指针再指向堆上的字符串内容。
        等于说是在中间加了一层指针包装。
    */
}
