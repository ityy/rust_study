//! Rust编写DLL或SO示例
//! #[no_mangle]表示方法名不打乱，方便调用方可以使用方法名直接调用
//! extern 表示方法可被外部调用

use std::collections::HashMap;

/// 01 无参无返回
#[no_mangle]
pub extern fn no_args_no_return() {
    println!("no_args_no_return is ok");
}

/// 02 有参无返回
#[no_mangle]
pub extern fn have_args_no_return(p1: u32, p2: &str) {
    println!("have_args_no_return is ok,args is p1:{} p2:{}", p1, p2);
}

/// 03 有参有返回，并回调主程序函数
#[no_mangle]
pub extern fn callback_test(func: fn(&str)) -> u32 {
    println!("from lib: callback_test is ok");
    func("Greetings from the dynamic library!");
    0
}