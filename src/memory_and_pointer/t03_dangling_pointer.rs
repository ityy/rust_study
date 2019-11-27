//! 悬垂指针 Dangling pointer

use std::collections::HashMap;
use std::rc::Rc;

//结构体内值的改变导致外部借用panic
#[test]
fn test_null_pointer() {
    struct Mapwarp {
        map: HashMap<String, i32>,
        count: i32,
    }

    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert("a1".to_string(), 1);
    map1.insert("b1".to_string(), 2);
    map1.insert("c1".to_string(), 3);

    let mut map_s = Mapwarp {
        map: map1,
        count: 0,
    };

    let map_b = &map_s.map;//普通借用

    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert("a2".to_string(), 4);
    map2.insert("b2".to_string(), 5);
    map2.insert("c2".to_string(), 6);
    map_s.map = map2;// 如果下方用到了map_b, 这一行就会报错:不能分配, 因为map_s.map被借用了且后面使用到了.
//    let end1 = map_b.get("a1").unwrap();
//    println!("{}", end1);
}

//使用引用计数解决悬垂指针的问题
#[test]
fn solve_null_pointer() {
    struct MapwarpRc {
        map: Rc<HashMap<String, i32>>,
        count: i32,
    }


    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert("a1".to_string(), 1);
    map1.insert("b1".to_string(), 2);
    map1.insert("c1".to_string(), 3);

    let mut map_s = MapwarpRc {
        map: Rc::new(map1),
        count: 0,
    };

    let map_b = map_s.map.clone();//引用计数借用

    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert("a2".to_string(), 4);
    map2.insert("b2".to_string(), 5);
    map2.insert("c2".to_string(), 6);
    map_s.map = Rc::new(map2);

    let end1 = map_b.get("a1").unwrap();
    println!("{}", end1);
}