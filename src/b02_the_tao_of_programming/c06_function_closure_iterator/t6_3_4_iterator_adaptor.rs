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
/// Map     对原始迭代器中的每一个元素调用指定的闭包来产生一个新的迭代器
/// Chain   通过连接两个迭代器产生一个新的迭代器
/// Cloned  拷贝原始迭代器来创建一个新的迭代器
/// Cycle   创建一个循环迭代器，每当迭代结束就返回第一个元素
/// Enumerate 创建一个计数迭代器，每次迭代会返回一个元组（i，val）i是索引，val是元素
/// Filter  创建一个过滤元素的迭代器，利用谓词判断（predicate，产生布尔值的表达式）
/// FlatMap 创建一个类似Map的迭代器，但不会包含任何嵌套
/// FliterMap 相当于依次调用Fliter和Map
/// Fuse（熔断） 创建一个快速迭代器，只要迭代过程遇到None那么之后的元素均为None
/// Rev     创建一个逆向遍历的迭代器
/// 以及其它自实现或第三方实现的迭代器适配器，比如社区中提供的itertools包，有很多强大的迭代器适配器。
#[test]
fn test() {
    //Map
    let arr1 = [1, 2, 3, 4, 5];
    //利用迭代器适配器，加工数组，并利用collect处理迭代器，收集到容器中。
    let c1 = arr1.iter().map(|x| 2 * x).collect::<Vec<i32>>();
    println!("{:?}", c1);
    //FliterMap
    let arr2 = ["1", "2", "3", "4", "5", "hello"];
    let c2 = arr2.iter().filter_map(|x| x.parse().ok()).collect::<Vec<i32>>();
    println!("{:?}", c2);
    //Enumerate
    let arr3 = ["a", "b", "c"];
    let c3 = arr3.iter().enumerate().collect::<Vec<(_, _)>>();
    println!("{:?}", c3);
}

/// 消费器（Consumer）
/// 迭代器适配器都是对next的包装增强，没有实际调用next进行消费的行为。Rust中的迭代器都是惰性的。
/// 最简单的消费行为就是for循环，其会隐性的调用next方法。
/// 为了便利性和高性能，Rust提供了一些for之外的消费器：
///     any     条件判断消费器     返回bool  查看容器中是否存在满足给定条件的元素
///     fold    积累计算消费器     返回单个值   fold（起始值，|上次迭代的结果acc，元素x| acc+x）
///                                         首次迭代时，上次迭代的结果为起始值。返回最后一次迭代的结果。
///     collect 收集结果消费器     返回集合    消费迭代器，并将每一个元素收集到指定的集合类型中。也成为收集器。
#[test]
fn test_any_fold() {
    let a = [1, 2, 3];
    let c = a.iter().any(|&x| x != 2);
    println!("{}", c);//true
    let sum = a.iter().fold(0, |acc, &x| acc + x);
    println!("{}", sum);//6
    let sum = a.iter().fold(0, |acc, &x| acc + (x * 2));
    println!("{}", sum);//12
}
