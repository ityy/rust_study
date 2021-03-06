//! # 函数
//! Rust中，使用fn关键字定义函数。函数名必须为蛇形命名法（snake_case）。
//! ```ignore
//! pub fn this_is_function(params: i32) -> i32 {//函数签名
//!     //函数体
//!     params //无分号的末尾表达式可直接视为返回值
//! }
//! ```
//! # 作用域与生命周期
//! Rust的作用域是静态作用域，即词法作用域，由一对花括号来表示作用域。<br/>
//! 其作用域在词法分析阶段就确立了，不会动态改变。<br/>
//! 变量的生命周期随着let绑定开始，随着作用域结束而结束。<br/>
//! ```ignore
//! pub fn scope() {
//!     let x = "hello";
//!     {
//!         let y = "world";
//!     }// y收回
//! }// x收回
//! ```

/// # 函数指针
/// 在Rust中，函数是一等公民。
/// 这表示函数也可以作为函数的参数和返回值使用。
#[test]
pub fn function_is_variable() {
    //函数内也支持定义函数
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    fn mul(a: i32, b: i32) -> i32 {
        a * b
    }

    //绑定到变量
    let sum_fn = sum;

    //定义一个函数，用函数作为参数。参数类型为函数的签名。
    fn math(operation: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        operation(a, b)
    }

    let result_sum = math(sum_fn, 1, 2);
    let result_mul = math(mul, 1, 2);
    println!("result_sum：{}", result_sum);
    println!("result_mul：{}", result_mul);

    //函数作为返回值
    fn is_true() -> bool {
        true
    }
    //函数作为返回值的函数
    fn true_maker() -> fn() -> bool {
        //返回一个函数
        is_true
    }
    println!("is_true：{}", true_maker()());//这里没问题，调用函数true_maker返回了一个函数，继续调用
}

/// CTFE机制 （编译时执行）
#[test]
fn ctfe_test() {
    //必须为const函数 否则：
    //error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
    // const函数：必须可以确定值，不能存在歧义。const函数可以在编译期执行。
    const fn init_len() -> usize {
        5
    }
    let arr = [0; init_len()];//编写时，数组长度为?号，表示不确定。编译时，init_len()就会被执行，数组arr的长度就会被确认。
    println!("array:{:#?}", arr);
    /*结果:
        array:[
        0,
        0,
        0,
        0,
        0,
        ]
    */
}