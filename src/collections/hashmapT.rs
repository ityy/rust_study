use std::collections::HashMap;

pub fn main() {

// 声明
    let mut come_from = HashMap::new();
// 插入
    come_from.insert("WaySLOG", "HeBei");
    come_from.insert("Marisa", "U.S.");
    come_from.insert("Mike", "HuoGuo");

// 查找key
    if !come_from.contains_key("elton") {
        println!("Oh, 我们查到了{}个人，但是可怜的Elton猫还是无家可归", come_from.len());
    }

// 根据key删除元素
    come_from.remove("Mike");
    println!("Mike猫的家乡不是火锅！不是火锅！不是火锅！虽然好吃！");

// 利用get的返回判断元素是否存在
    let who = ["MoGu", "Marisa"];
    for person in &who {
        match come_from.get(person) {
            Some(location) => println!("{} 来自: {}", person, location),
            None => println!("{} 也无家可归啊.", person),
        }
    }

// 遍历输出
    println!("那么，所有人呢？");
    for (name, location) in &come_from {
        println!("{}来自: {}", name, location);
    }
}