//! 数据类型


/// 01 bool 类型
/// bool可以转为整数，但整数不能转为bool
#[test]
fn test_bool() {
    let x = true;
    let y = false;
    println!("true is {}", x as i32);
    println!("false is {}", y as i32);
}

/// 02 数字类型
#[test]
fn test_digit() {
    let x = 42; //默认为i32
    let x = 42u8;
    let x = 42i8;
    let x = 42u16;
    let x = 42i16;
    let x = 42u32;
    let x = 42i32;
    let x = 42u64;
    let x = 42i64;

    let num = 0x1F; //16进制
    let num = 0o17; //8进制
    let num = 0b11; //2进制
    let num = 0b1011_1101; //2进制,支持分隔符_
    let num = b'a'; //字节编码值
}

/// 03 字符类型
/// 字符类型代表一个Unicode标量值，占用4个字节。
#[test]
fn test_char() {
    let x = 'x';
    println!("{}", x);
    println!("0x{:X}", x as u8);//0x78

    //可以直接用ASCII码和Unicode码来定义字符类型
    let a = '\x61'; //支持 '\xHH' 格式的ASCII码
    println!("{}", a);//a
    let u = '\u{151}'; //支持 '\u{HHH}' 格式的Unicode码
    println!("{}", u);//ő

    //字符转整数
    println!("{}", a as u8);//97
    println!("{}", u as u8);//81 (高位被截断)
}

/// 04 数组类型
/// 1 数组大小固定
/// 2 数组元素类型相同
/// 3 默认不可变
#[test]
fn test_array() {
    //定义方法：[类型；长度]
    let x: [i32; 3] = [1, 2, 3];
    println!("{:?}", x);
}

/// 05 范围类型
/// Rust内置了范围（Range）类型，包括[左闭右开),[全闭]两种区间。
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
    // Rust提供了非常便捷的语法糖: 范围语法
    let x = 1..3;// Range的简易创建形式
    let y = 1..=3;//RangeInclusive的简易创建形式
}


/// 06 切片(slice)类型
/// 切片是对一个数组片段的引用。
/// 用[T]表示数组，则切片类型就是&[T],&mut [T]。
/// 使用范围语法指定切片大小
/// 范围语法:
///     x..y    x到y 不含y
///     x..=y   x到y 含y
///     x..     x到结束
///     ..y     开头到y
///     ..      开头到结束
#[test]
fn test_slice() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);//内容相等
    assert_eq!(&arr[1..], [2, 3, 4, 5]);//内容相等
    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1, 2, 3])
}

/// 07 字符串
/// 出于安全考虑，Rust将字符串分为两种类型：
/// 1 &str 切片引用 固定长度
/// 2 String 结构体 可变长度
#[test]
fn test_string() {
    let truth: &'static str = "Rust是一门优雅的语言";
    let len = truth.len();
    println!("{}", len);//28
    let ptr = truth.as_ptr();
    let s = unsafe {
        //以原生指针+长度 创建字节切片引用&[u8]
        //Rust中字符串的本质是一段有效的UTF-8字节序列
        let slice = std::slice::from_raw_parts(ptr, len);
        let s = std::str::from_utf8(slice);
        s
    };
    println!("{}", s.unwrap());
}

/// 08 原生指针
/// Rust提供了多种指针，包括引用（Reference）、原生指针（Raw Pointer）、函数指针（fn Pointer）、智能指针（Smart Pointer)
/// Rust分为Safe Rust和Unsafe Rust。
/// 引用主要用于Safe Rust，原生指针主要用于Unsafe Rust。
/// 原生指针也分为:
///     不可变原生指针*const T。
///     可变原生指针*mut T。
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
        *x_mut_ptr += *y_mut_ptr;
        println!("{}", *x_mut_ptr);
    }
}

/// 09 never类型
/// Rust提供了一种特殊的类型，never类型，即！。
/// 该类型表示永远不可能有返回值的计算类型。
/// Rust分为Safe Rust和Unsafe Rust。
/// 引用主要用于Safe Rust，原生指针主要用于Unsafe Rust。
/// 原生指针也分为:
///     不可变原生指针*const T。
///     可变原生指针*mut T。
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