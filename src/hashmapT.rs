///map 学习.
use core::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

struct LibInfo {
    libmap: HashMap<String, String>,
}

//静态变量只能用于如下类型
//calls in statics are limited to constant functions, tuple structs and tuple variants
//static mut LIB_INFO: LibInfo = LibInfo { libmap: HashMap::new() };

pub fn main() {
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
    // 因为 get 返回 Option<V>，所以结果被装进 Some；如果某个键在哈希 map 中没有对应的值，get 会返回 None。

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