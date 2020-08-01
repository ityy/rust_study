//! # 方法
//! 方法语义来自面向对象编程范式，它代表某个对象的行为，函数只是一段简单的代码。<br/>
//! 方法必须关联一个方法接收者。


/// # 定义结构体
#[derive(Debug)]
struct User {
    name: &'static str,
    avatar_url: &'static str,
}

/// # 为结构体实现方法
impl User {
    fn show(&self) {
        println!("name:{:?}", self.name);
        println!("avatar:{:?}", self.avatar_url);
    }
}

/// # 方法使用示例
#[test]
fn test_method() {
    let user = User {
        name: "明洋",
        avatar_url: "www.baidu.com",
    };

    //以下两种调用等价
    user.show();
    User::show(&user);
}