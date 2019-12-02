//! 类型推导
//! Rust只能在局部范围内进行类型推导。
//!
//! Turbofish 操作符
//! Rust提供::<>用于标注类型，称为Turbofish操作符

/// 测试类型推导
#[test]
fn test_type_infer() {
    let a = 1;
    let b = 2;
    fn sum(x: u32, y: u32) -> u32 {
        x + y
    }
    sum(a, b);//在这里，a b被推导为u32

    let mut vec = Vec::new(); //此时vec为<_>类型，_是通配符，表示占位
    vec.push(1);//此时vec被推导为Vec<i32>类型
}

/// 测试Turbofish 操作符
#[test]
fn test_turbofish() {
    //以字符串转换为例
    let x = "1";
    //println!("{}", x.parse().unwrap()); //error[E0284]: type annotations required 由于整数类型很多，编译器不能确定程序员的意图，需要明确类型
    println!("{}", x.parse::<i32>().unwrap()); //parse是一个泛型方法，可以使用Turbofish操作符指定方法的泛型类型。
    let y: i32 = x.parse().unwrap();//也可以明确y的类型，编译器会自动推导parse方法的泛型类型
}
