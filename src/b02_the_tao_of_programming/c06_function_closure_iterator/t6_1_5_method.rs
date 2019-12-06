//! 方法
//! 方法语义来自面向对象编程范式，它代表某个对象的行为，函数只是一段简单的代码。
//! 方法必须关联一个方法接收者。

/// 定义结构体，并实现一些方法
#[test]
fn test_method() {
    #[derive(Debug)]
    struct User {
        name: &'static str,
        avatar_url: &'static str,
    }
    impl User {
        fn show(&self) {
            println!("name:{:?}", self.name);
            println!("avatar:{:?}", self.avatar_url);
        }
    }

    let user = User {
        name: "明洋",
        avatar_url: "www.baidu.com",
    };

    //以下两种调用等价
    user.show();
    User::show(&user);
}