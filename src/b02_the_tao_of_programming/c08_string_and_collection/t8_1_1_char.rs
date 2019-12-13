//! 字符 char
//! Rust使用char表示单个字符，char类型使用整数值与Unicode标量值一一对应。
//! Rust中的char统一占用4个字节。
//! 与char不同，Rust中String使用的是UTF-8的编码序列，且不支持其它编码。

#[test]
fn test_char() {
    let tao: char = '道';
    let tao_unicode = tao as u32;
    println!("0x{:X}", tao_unicode);// 9053
    //通过char类型的escape_unicode方法可以获取标量值
    println!("{}", tao.escape_unicode());// \u{9053}

    let tao: char = std::char::from_u32(0x9053).unwrap();
    println!("{}", tao);
    let tao: String = String::from("\u{9053}"); //Rust下Unicode码的书写方式
    println!("{}", tao);
}
