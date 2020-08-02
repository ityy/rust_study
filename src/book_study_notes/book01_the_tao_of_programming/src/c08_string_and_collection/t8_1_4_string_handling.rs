//! # 字符串的处理
//! 由于Rust使用UTF-8保存字符串，其动态长度的特性使得无法使用索引访问字符串中的字符。<br/>
//! 为了解决这个问题，Rust提供了两个迭代器：
//! - bytes 按字节迭代字符串
//! - chars 按字符迭代字符串

/// # 字符串迭代器演示
#[test]
fn test_string() {
    // 字符迭代器演示
    let s = "borös";
    let mut chars = s.chars();
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('r'), chars.next());
    assert_eq!(Some('ö'), chars.next());
    assert_eq!(Some('s'), chars.next());

    // 字节迭代器演示
    let mut bytes = s.bytes();
    assert_eq!(6, s.len()); // 注意，len()返回的是字节长度，不是字符长度
    assert_eq!(Some(98), bytes.next());
    assert_eq!(Some(111), bytes.next());
    assert_eq!(Some(114), bytes.next());
    assert_eq!(Some(195), bytes.next());
    assert_eq!(Some(182), bytes.next());
    assert_eq!(Some(115), bytes.next());
}