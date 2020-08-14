//! # 基本数据类型


/// ## 01 bool 类型
/// bool可以转为整数，但整数不能转为bool
#[test]
fn test_bool() {
    let x = true;
    let y = false;
    println!("true is {}", x as i32);// 1
    println!("false is {}", y as i32);// 0
}


/// ## 02 数字类型
#[test]
fn test_digit() {
    // 字宽
    let x = 42; //默认为i32
    let x = 42u8;
    let x = 42i8;
    let x = 42u16;
    let x = 42i16;
    let x = 42u32;
    let x = 42i32;
    let x = 42u64;
    let x = 42i64;

    // 进制
    let num = 0x1F; //16进制
    let num = 0o17; //8进制
    let num = 0b11; //2进制
    let num = 0b1011_1101; //2进制,支持分隔符_
    let num = b'a'; //字节编码值
}

/// ## 03 字符类型
/// 字符类型代表一个Unicode标量值，占用4个字节。
/// 标量值均存储在栈空间内。
#[test]
fn test_char() {
    // 代码编译为指令后，字符a作为指令的操作数。
    // 代码运行后，x表示栈上的一个地址，指令直接将操作数字符a赋值给了x。
    let x = 'a';
    let y = '杨';
    println!("x code is 0x{:X}", x as u32);//0x61  {:X}表示按16进制大写格式化
    println!("y code is 0x{:X}", y as u32);//0x6768 如果是u8，则高位会被截断
    print_addr_and_value(&x as *const char as usize, 4);
    print_addr_and_value(&y as *const char as usize, 4);
    /*X86为小端模式，即权值小的数在低地址处。
    验证明确Rust中的char均为4字节unicode编码。
    addr:0x270DFF088 value:0x61
    addr:0x270DFF089 value:0x0
    addr:0x270DFF08A value:0x0
    addr:0x270DFF08B value:0x0

    addr:0x270DFF08C value:0x68
    addr:0x270DFF08D value:0x67
    addr:0x270DFF08E value:0x0
    addr:0x270DFF08F value:0x0
     */

    //可以直接用ASCII码和Unicode码来定义字符类型
    let a = '\x61'; //支持 '\xHH' 格式的ASCII码
    println!("{}", a);//a
    let b = '\u{6768}'; //支持 '\u{HHH}' 格式的Unicode码
    println!("{}", b);//杨
}


/// ## 04 数组类型
/// - 数组大小固定
/// - 数组元素类型相同
/// - 默认不可变
#[test]
fn test_array() {
    //数组类型定义方式：
    // 方式1：指定数组类型：[类型；长度]
    let x: [i32; 3] = [1, 2, 3];
    println!("{:?}", x);

    // 方式2：初始化为长度5，值为1的数组。
    let y = [1; 5];
    println!("{:?}", y);
}

/// ## 05 范围类型
/// Rust内置了范围（Range）类型，包括 [左闭右开)、[全闭] 两种区间。
///
/// Range是标准库里的一个结构体，有两个属性, start和end：
/// ```
/// pub struct Range<Idx> {
///     /// The lower bound of the range (inclusive).
///     #[stable(feature = "rust1", since = "1.0.0")]
///     pub start: Idx,
///     /// The upper bound of the range (exclusive).
///     #[stable(feature = "rust1", since = "1.0.0")]
///     pub end: Idx,
/// }
/// ```
/// rust提供了一个语法糖, 可以快速创建range，只需要使用(x..y)的形式, 即可实现一个 [x,y) 左闭右开的区间。
#[test]
fn test_range() {
    // 范围类型有两个struct：
    // [左闭右开)
    let x = std::ops::Range {
        start: 1,
        end: 3,
    };
    // [全闭]
    let y = std::ops::RangeInclusive::new(1, 3);

    //语法糖 快速创建Range
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));

    //Range相关的方法
    assert_eq!((3..6).max().unwrap(), 5);
    assert_eq!((3..=6).sum::<u32>(), 3 + 4 + 5 + 6);
}


/// ## 06 切片(slice)类型
/// 切片是对一个数组片段的引用。<br/>
/// 用[T]表示数组，则切片类型就是&[T],&mut [T]。<br/>
/// 可以使用范围语法指定切片大小。
/// ### 范围语法:
/// >- x..y    x到y 不含y
/// >- x..=y   x到y 含y
/// >- x..     x到结束
/// >- ..y     开头到y
/// >- ..      开头到结束
#[test]
fn test_slice() {
    // 定义数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);//内容相等
    assert_eq!(&arr[1..], [2, 3, 4, 5]);//内容相等
    // 定义vec
    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1, 2, 3])
}

/// ## 07 字符串
/// 出于安全考虑，Rust将字符串分为两种类型：
/// 1. &str 切片引用 固定长度
/// 2. String 结构体 可变长度
#[test]
fn test_string() {
    let truth: &'static str = "abcdabcd杨洋你好";//utf-8 总占20字节（字母1字节，汉字3字节）
    println!("truth value addr：{:p}", truth); //
    println!("truth addr：{:p}", &truth);
    let len = truth.len();
    println!("truth.len() is {}", len);//20 长度为所占字节数

    // 获取字符串的内存地址，并打印内存观察。
    // 方式一
    // truth不能直接转为原生指针再转为usize，可以取其地址直接按层递进操作内存即可。简述为获取变量的地址，将其从新描述为usize类型的地址后取值，即可得到变量中存储的地址。
    // convert_addr_to_int(truth); //error[E0277]: the size for values of type `str` cannot be known at compilation time 编译时不知道str的大小。向上一层找&truth代替。
    // 1 获取变量内存地址
    let truth_addr = convert_addr_to_int(&truth);
    // 观察内存结构 结论：此类型的变量truth描述了16字节的内存信息，前8字节为字符串在堆上的地址，后8字节为字符串的长度，长度单位为字节数。
    print_addr_and_value(truth_addr, 16);
    // 2 将此地址转为u64指针，以通过此指针获取u64的目标信息。
    let truth_ptr = truth_addr as *const usize;
    // 获取指针内的地址
    let truth_ptr_value = unsafe {
        println!("truth_ptr:{:X}", *truth_ptr);
        *truth_ptr
    };
    // 观察内存结构 结论：与char类型固定4字节长度不同，字符串使用utf8格式存储。所以字符长度并不固定，由1-4字节浮动。
    print_addr_and_value(truth_ptr_value, 20);

    //方式二
    //智能指针大多提供了as_ptr()转为一个u8类型指针的方法。
    let ptr = truth.as_ptr();//获取指针，单字节类型
    println!("通过truth.as_ptr() 获取首字节地址：{:p}", ptr);
    println!("将首字节地址转为usize：{:X}", ptr as usize);
    let s = unsafe {
        //以原生指针+长度，创建字节切片引用&[u8]。其返回一个u8数组的引用。
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    println!("通过truth.as_ptr() 操作字符串：{}", s.unwrap());
}

/// ## 08 原生指针
/// Rust提供了多种指针，包括引用（Reference）、原生指针（Raw Pointer）、函数指针（fn Pointer）、智能指针（Smart Pointer)<br/>
/// Rust分为Safe Rust和Unsafe Rust。<br/>
/// 引用主要用于Safe Rust，原生指针主要用于Unsafe Rust。<br/>
/// 原生指针也分为:<br/>
///  - 不可变原生指针*const T。
///  - 可变原生指针*mut T。
#[test]
fn test_pointer() {
    let mut x = 10;
    //获取x的指针
    let x_mut_ptr = &mut x as *mut i32;
    //包装x，此时所有权已转移给y，x作废
    let mut y = Box::new(x);
    //获取y的指针 注意要解一层Box
    let y_mut_ptr = &mut *y as *mut i32;
    //通过指针强制访问作废的x
    unsafe {
        *x_mut_ptr += *y_mut_ptr;//10+10
        println!("{}", *x_mut_ptr);//20
    }
}

/// ## 09 never类型
/// Rust提供了一种特殊的类型，never类型，即！。
///
/// 该类型表示永远不可能有返回值的计算类型。
#[test]
fn test_never() {
    let num: Option<i32> = Some(42);
    let x = match num {
        //返回i32类型
        Some(x) => x,
        //返回！类型。这里类型不同但没有报错，是因为never类型可以强转为任何类型。
        None => panic!("nothing!"),
    };
}


/// ## 打印内存
/// 入参（起始地址，字节数）
pub fn print_addr_and_value(addr_begin: usize, byte_count: usize) {
    //需要访问原生指针，则必须在unsafe块中使用
    unsafe {
        for i in 0..byte_count {
            let x = (addr_begin + i) as *const u8;//将整数型地址，转为u8型raw指针（此处要打印字节，所以需要u8型指针）。
            println!("addr:0x{:X} value:0x{:X}", addr_begin + i, *x); //取指针x存储的数据
        }
    }
}

/// ## 内存地址格式转换
/// 将T类型的Rust引用型（安全指针类型，即安全内存地址类型）转为整数类型
pub fn convert_addr_to_int<T>(t: &T) -> usize {
    //rust引用和raw指针 raw指针和usize 可以互相转换, 这就为内存操作提供了无限可能
    //*const表示原生不可变指针， *mut表示原生可变指针。
    t as *const T as usize
}