//! # 字符串的处理
//! 由于Rust使用UTF-8保存字符串，其动态长度的特性使得无法使用索引访问字符串中的字符。<br/>
//! 为了解决这个问题，Rust提供了两个迭代器：
//! - bytes 按字节迭代字符串
//! - chars 按字符迭代字符串

/// # 字符串迭代器演示
#[test]
fn test_string() {
    // 单字符迭代
    let s = "borös";
    let mut chars = s.chars();
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('r'), chars.next());
    assert_eq!(Some('ö'), chars.next());
    assert_eq!(Some('s'), chars.next());

    // 单字节迭代
    let mut bytes = s.bytes();
    assert_eq!(6, s.len()); // 注意，len()返回的是字节长度，不是字符长度
    assert_eq!(Some(98), bytes.next());
    assert_eq!(Some(111), bytes.next());
    assert_eq!(Some(114), bytes.next());
    assert_eq!(Some(195), bytes.next());
    assert_eq!(Some(182), bytes.next());
    assert_eq!(Some(115), bytes.next());
}


fn string_test() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
//    let s3 = s1.add(&s2);
//    let s3 = s1 + &s2;
    let s3 = String::from("bbb") + "ddd";
    println!("{}", s3);

    let x = "hello"; //等价下句
    let x: &'static str = "world"; //等价上句 字符串字面量是写死到代码里的,有固定访问地址,是全局变量
}

#[test]
fn string_slice_test() {
    //原始字符串 存放着字符串的内存地址和长度
    let str_raw: &'static str = "rust 是一门优雅的语言";
    //获取C指针
    let raw_ptr = str_raw.as_ptr();
    //获取长度
    let len = str_raw.len();

    let addr = &str_raw as *const &str as usize;
    println!("raw_ptr：0x{:X}", raw_ptr as usize);

    println!("addr：0x{:X}", addr);

    let x = addr as *mut &str;
    unsafe {
        let n = &**x; //**X拿到了原始数据, 但因为不能move, 必须使用引用, 使用&再生成一个引用地址
        println!("n_addr：{:p}", n);
    }


    //通过指针的方式 获取数据切片
    let s = unsafe {
        //slice为一个字节数组
        let slice = std::slice::from_raw_parts(raw_ptr, len);
        std::str::from_utf8(slice)
    };
    println!("raw_str:{}", s.unwrap());
}


/// # 切片测试
/// 字符串少用切片, 会切出不完整的unicode编码导致报错
#[test]
pub fn slice_test() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{},{}", hello, world);
    assert_eq!("hello", hello);
}
