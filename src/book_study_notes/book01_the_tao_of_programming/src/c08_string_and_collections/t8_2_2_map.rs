//! # Map 映射集
//! Map是常用的数据结构，依照键值对进行存储。每个键值对被称为一个Entry（条目，入口）。 <br/>
//! Map中不能存在重复的key。 <br/>
//! Map提供查找，插入和删除的时间复杂度是O(1),最坏情况也是O(N)。属于空间换时间的做法。 <br/>
//! Rust提供了两种Map：
//! -   HashMap<K,V>    基于哈希表的无序Key-Value映射集
//! -   BTreeMap<K,V>   基于B树的有序Key-Value映射集，按Key排列

use core::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

/// # HashMap的CRUD
#[test]
fn test_crud() {
    // 创建一个map，存放书本评语
    let mut book_reviews = HashMap::with_capacity(10);
    book_reviews.insert("Rust Book", "good");
    book_reviews.insert("Programming Rust", "nice");
    book_reviews.insert("The Tao Of Rust", "deep");

    // 遍历map
    for (k, v) in &book_reviews {
        println!("book_name:{} review:{}", k, v);
    }

    // 只遍历key
    for key in book_reviews.keys() {
        println!("{}", key);
    }

    // 只遍历value
    for value in book_reviews.values() {
        println!("{}", value);
    }

    // 判断是否包含指定key
    if !book_reviews.contains_key("葫芦娃") {
        println!("find {} times", book_reviews.len());
    }

    // 根据key 移出元素（包括所有权）
    let x = book_reviews.remove("Rust Book");
    match x {
        Some(text) => { println!("Rust Book：{}", text); }
        None => { println!("不存在 Rust Book"); }
    }

    // 根据key 获取值的引用
    let x = book_reviews.get("The Tao Of Rust");
    println!("The Tao Of Rust：{}", x.unwrap());

    // 根据key 获取值的可变引用
    let x = book_reviews.get_mut("The Tao Of Rust");
    println!("The Tao Of Rust：{}", x.unwrap());

    // Index语法模式： object[index]
    // 以模拟'数组[下标]'的方式快速操作目标。
    // 实现Index特性的数据结构都可以使用这种语法读。
    // 实现IndexMut特性的数据结构都可以通过这种语法读、写。
    // HashMap尚未实现IndexMut特性。
    println!("{}", book_reviews["The Tao Of Rust"]);
}

/// # Entry模式
/// 对于HashMap中的单个桶（Bucket）来说，其状态无非是“空”和“满”，Rust对此做了一层抽象，使用Entry枚举来表示每个键值对。
///
/// Entry有两个值，一个表示满，一个表示空。
/// ```
/// #[stable(feature = "rust1", since = "1.0.0")]
/// pub enum Entry<'a, K: 'a, V: 'a> {
///     /// An occupied entry.
///     #[stable(feature = "rust1", since = "1.0.0")]
///     Occupied(#[stable(feature = "rust1", since = "1.0.0")] OccupiedEntry<'a, K, V>),
///
///     /// A vacant entry.
///     #[stable(feature = "rust1", since = "1.0.0")]
///     Vacant(#[stable(feature = "rust1", since = "1.0.0")] VacantEntry<'a, K, V>),
/// }
/// ```
#[test]
fn test_entry() {
    let mut map = HashMap::<&str, u32>::new();

    // 取指定key的值，无则插入后再取。
    let _x = map.entry("current_year").or_insert(2017);
    assert_eq!(map["current_year"], 2017);

    *map.entry("current_year").or_insert(2017) += 10;
    assert_eq!(map["current_year"], 2027);

    *map.entry("next_year").or_insert_with(|| 2018) += 10;
    assert_eq!(map["next_year"], 2028);

    assert_eq!(&"next_year", map.entry("next_year").key());
}


/// # HashMap 实现原理
/// 不管哪门语言，实现HashMap的过程可以分为三步：
/// 1. 实现一个Hash函数
/// 2. 合理解决Hash冲突
/// 3. 实现HashMap的操作方法
///
/// ```
/// HashMap底层是基于数组的，当插入键值对时，通过对k进行Hash运算得到Hash值，再模除数组长度，得到具体位置。
/// 整个过程最重要的是Hash函数。
/// 一个好的Hash函数，不仅性能优越，还会让储存于底层数组中的值分布的更加均匀，减少冲突的发生。
/// Hash碰撞（Hash Collision） 即Hash冲突。
///     取得相同Hash值的两个不同元素，称为同义词，发生了碰撞。
/// 负载因子（Load Factor） 也是发生碰撞的决定性因素之一。
///     键值对与容量的比值，比如容量100，已存储90个键值对，则负载因子为90/100，即0.9。
///     负载因子决定容器什么时候进行扩容。
/// Rust实现HashMap采用的Hash函数算法是SipHash13算法（默认，性能更佳）或SipHash24算法（更安全）。
/// SipHash可以防止Hash碰撞拒绝服务攻击（Hash Collision DoS）。
/// 这种攻击是一种针对Hash函数的特性构造出的增强碰撞的手段，被攻击的CPU占用率会飙升至100%，从而导致服务不可用。
/// ```
fn nothing() {}

/// # 结构体中的HashMap
struct LibInfo {
    libmap: HashMap<String, String>,
}

/// # 静态变量
/// ```
/// static mut LIB_INFO: LibInfo = LibInfo { libmap: HashMap::new() };
/// // error: calls in statics are limited to constant functions, tuple structs and tuple variants
/// ```
#[test]
pub fn test_struct() {
    //新建
    let mut map: HashMap<String, i32> = HashMap::new();

    //插入数据
    map.insert(String::from("R"), 255);
    map.insert(String::from("G"), 255);
    map.insert(String::from("B"), 255);
    let tuple = ("ddd", 1);

    //通过zip创建
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50, 30];
    let zip = teams.iter().zip(initial_scores.iter());
    let scores: HashMap<_, _> = zip.collect();//收集为HashMap
    let s1 = String::from("Blue");
    let i1 = scores.get(&s1).unwrap();
    println!("{}", i1); //打印结果: 10

    // 所有权
    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者。
    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。

    // 访问数据
    // 因为 get 返回 Option<V>，所以结果被装进 Some；如果某个键在 map 中没有对应的值，get 会返回 None。

    // 遍历
    for (k, v) in &map {
        println!("{}:{}", k, v); //这会以任意顺序打印出每一个键值对
    }

//    test_static();
//    test_static2();


    let mut map_struct = Mapwarp {
        map: HashMap::new(),
        count: 0,
    };
//    let mut map_struct_mut_map = map_struct.map;
    map_struct.map.insert(String::from("X"), 255);
    map_struct_mut_map_test(&map_struct);
    // 遍历
//    for (k, v) in map_struct_mut_map {
//        println!("{}:{}", k, v); //这会以任意顺序打印出每一个键值对
//    }
}

fn map_struct_mut_map_test(map: &Mapwarp) {
    let map_struct_mut_map = map.map.get(String::from("X").as_str());
    println!("{}", map_struct_mut_map.unwrap());
}

//fn test_static() {
//    unsafe {
//        LIB_INFO.libmap.insert("1".to_string(), "a".to_string());
//    }
//}

//fn test_static2() {
//    unsafe {
//        let s = LIB_INFO.libmap.get("1").unwrap();
//        println!("static map:{}", s);
//    }
//}


//有Mapwarp的所有权才可以获取HashMap的可变借用
//有HashMap的可变借用才可以插入数据
struct Mapwarp {
    map: HashMap<String, i32>,
    count: i32,
}


