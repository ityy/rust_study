//! # 悬垂指针 Dangling pointer

use std::collections::HashMap;
use std::rc::Rc;

struct Mapwarp {
    map: HashMap<String, i32>,
    count: i32,
}


/// # 悬垂指针
/// 使用悬垂指针，编译器提前报错
#[test]
fn test_null_pointer() {
    let (map1, map2) = get_two_map();

    let mut mapwarp = Mapwarp {
        map: map1,
        count: 0,
    };
    //普通借用 &mapwarp.map 等价 &(mapwarp.map)
    let map1_ref = &mapwarp.map;

    mapwarp.map = map2;// 释放map1，并获得map2，同时map1_ref失效

    // map1_ref.get("a1").unwrap();// map1_ref成为悬垂指针，编译器会报错
}

struct MapwarpRc {
    map: Rc<HashMap<String, i32>>,
    count: i32,
}

/// # 使用引用计数解决悬垂指针问题
#[test]
fn solve_null_pointer() {
    let (map1, map2) = get_two_map();

    let mut mapwarp_rc = MapwarpRc {
        map: Rc::new(map1),
        count: 0,
    };

    let map1_ref = mapwarp_rc.map.clone();//引用计数借用

    mapwarp_rc.map = Rc::new(map2);

    let value = map1_ref.get("a1").unwrap();
    println!("{}", value);
}


fn get_two_map() -> (HashMap<String, i32>, HashMap<String, i32>) {
    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert("a1".to_string(), 1);
    map1.insert("a2".to_string(), 2);
    map1.insert("a3".to_string(), 3);

    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert("b1".to_string(), 4);
    map2.insert("b2".to_string(), 5);
    map2.insert("b3".to_string(), 6);

    (map1, map2)
}
