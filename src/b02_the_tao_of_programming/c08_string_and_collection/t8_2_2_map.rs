//! Map 映射集
//! Map是常用的数据结构，依照键值对进行存储。每个键值对被称为一个Entry（条目，入口）.
//! Map中不能存在重复的key
//! Map提供查找，插入和删除的时间复杂度是O(1),最坏情况也是O(N)。属于空间换时间的做法。
//! Rust提供了两种Map：
//!     HashMap<K,V>    基于哈希表的无序Key-Value映射集
//!     BTreeMap<K,V>   基于B树的有序Key-Value映射集，按Key排列

use std::collections::HashMap;

/// HashMap的CRUD
#[test]
fn test() {
    // 书评
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
    book_reviews.remove("Rust Book");
    // 根据key 获取值的引用
    let x = book_reviews.get("The Tao Of Rust");
    println!("{}", x.unwrap());
    // 根据key 获取值的可变引用
    let x = book_reviews.get_mut("The Tao Of Rust");
    println!("{}", x.unwrap());
    // 快捷语法 Index语法模式： object[index]
    // 实现Index特性的数据结构都可以使用这种语法读
    // 实现IndexMut特性的数据结构都可以通过这种语法读和写
    // HashMap尚未实现IndexMut特性
    println!("{}", book_reviews["The Tao Of Rust"]);
}

/// #Entry模式
/// 对于HashMap中的单个桶（Bucket）来说，其状态无非是“空”和“满”，Rust对此做了一层抽象，使用Entry枚举来表示每个键值对。
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
    map.entry("current_year").or_insert(2017);
    assert_eq!(map["current_year"], 2017);

    *map.entry("current_year").or_insert(2017) += 10;
    assert_eq!(map["current_year"], 2027);

    *map.entry("next_year").or_insert_with(|| 2018) += 10;
    assert_eq!(map["next_year"], 2028);

    assert_eq!(&"next_year", map.entry("next_year").key());
}


/// HashMap 实现原理
/// 不管哪门语言，实现HashMap的过程可以分为三步：
///     1 实现一个Hash函数
///     2 合理解决Hash冲突
///     3 实现HashMap的操作方法
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
fn nothing() {}