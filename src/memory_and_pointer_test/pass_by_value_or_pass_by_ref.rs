//! # Rust 值传递、引用传递研究
//!


/// # 值传递
#[test]
fn pass_by_value() {
    let mut string_hello = String::from("hello");
    let mut string_world = String::from("world");
    let mut string_hello_mut_ref = &mut string_hello;
    let mut string_world_mut_ref = &mut string_world;

    println!("string_hello_mut_ref 地址 {:p},值 {:p}", &string_hello_mut_ref, string_hello_mut_ref);
    println!("string_world_mut_ref 地址 {:p},值 {:p}", &string_world_mut_ref, string_world_mut_ref);

    /// # 值传递
    /// 函数的内部变量是一个新的栈地址，值为调用函数时的实参表达式所求得的。
    fn change_ref<'a>(mut inner_var_x: &'a mut String, mut inner_var_y: &'a mut String) {
        println!("inner_var_x 地址 {:p},值 {:p}", &inner_var_x, inner_var_x);
        println!("inner_var_y 地址 {:p},值 {:p}", &inner_var_y, inner_var_y);
        inner_var_x = inner_var_y;
        println!("inner_var_x 地址 {:p},值 {:p}", &inner_var_x, inner_var_x);
    }

    change_ref(string_hello_mut_ref, string_world_mut_ref);

    println!("string_hello_mut_ref 地址 {:p},值 {:p}", &string_hello_mut_ref, string_hello_mut_ref);
    println!("{} {}", string_hello_mut_ref, string_world_mut_ref)// 结果是 hello world 而不是 world world
}


/// # 引用传递
#[test]
fn pass_by_ref() {}


