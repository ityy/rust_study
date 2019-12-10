//! 迭代器适配器
//! 迭代器提供了Map、Filter等适配器，这里有点类似java的流式处理，但Rust的实现更加具体，更能体会到本质。
//! 这些适配器，本质上都是迭代器的包装器（Wrapper）。适配器模式，也称为包装器模式。

/// 展示Map为迭代器的适配器
#[test]
fn test_map() {
    let a = vec![1, 2, 3];
    /*
        map返回了一个Map类型，Map是一个迭代器的包装器，本质上也是一个迭代器。
        Map利用传入的闭包对原来迭代器的next方法做了增强。
        所以返回一个新的迭代器，当调用next方法迭代时，已经是被闭包所影响的结果了。
    */
    let mut iter = a.into_iter().map(|x| 2 * x);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
}

/// 其它常用适配器说明：
///
