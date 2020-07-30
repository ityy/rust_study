//! 简单测试主程序
//! 使用单独函数直接访问的形式测试动态库
extern crate libloading as lib;

use std::error::Error;

/// 01 无参无返回
fn call_no_args_no_return() -> Result<u32, Box<dyn Error>> {
    println!("call_no_args_no_return start...");
    // 相对目录说明：
    // 在IDE中运行时，当前相对目录就是当前子项目源码的根目录。此处即为：simple_main_project/目录下。
    // 所以使用以下相对路径可以找到编译后的动态库。
    let lib = lib::Library::new(r"../../../../target/debug/dylib_1.dll")?;
    unsafe {
        let lib_func: lib::Symbol<unsafe extern fn()> = lib.get(b"no_args_no_return")?;
        lib_func();
    }
    println!("call_no_args_no_return end\n");
    Ok(0)
}

/// 02 有参无返回
fn call_have_args_no_return() -> Result<u32, Box<dyn Error>> {
    println!("call_have_args_no_return start...");
    let lib = lib::Library::new(r"../../../../target/debug/dylib_1.dll")?;
    unsafe {
        let lib_func: lib::Symbol<unsafe extern fn(p1: u32, p2: &str)> = lib.get(b"have_args_no_return")?;
        lib_func(10000, "hello lib!");
    }
    println!("call_have_args_no_return end\n");
    Ok(0)
}


/// 03 有参有返回，并回调主程序函数
fn call_callback_test() -> Result<u32, Box<dyn Error>> {
    println!("call_callback_test start...");
    let lib = lib::Library::new(r"../../../../target/debug/dylib_1.dll")?;
    let result: u32 = unsafe {
        let lib_func: lib::Symbol<unsafe extern fn(fn(&str)) -> u32> = lib.get(b"callback_test")?;
        lib_func(callback_test)
    };
    println!("call_callback_test end\n");
    Ok(result)
}

fn callback_test(message: &str) {
    println!("from main: execute callback_test args is: {}", message);
}

/// 04 测试动态库中的框架
fn call_reqwest_test() -> Result<u32, Box<dyn Error>> {
    println!("call_reqwest_test start...");
    let lib = lib::Library::new(r"../../../../target/debug/dylib_2.dll")?;
    unsafe {
        let lib_func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"reqwest_test")?;
        lib_func();
    }
    println!("call_reqwest_test end\n");
    Ok(0)
}


/// 命令行执行时的命令参数：
/// C:/Users/admin/.cargo/bin/cargo.exe test --color=always --package simple_main_project --bin simple_main_project dylib_1_test --no-fail-fast -- --exact -Z unstable-options --format=json --show-output
#[test]
fn dylib_1_test() {
    call_no_args_no_return();
    call_have_args_no_return();
    let r = call_callback_test();
    println!("call_callback_test result is {}", r.unwrap());
}

/// 命令行执行时的命令参数：
/// C:/Users/admin/.cargo/bin/cargo.exe test --color=always --package simple_main_project --bin simple_main_project dylib_2_test --no-fail-fast -- --exact -Z unstable-options --format=json --show-output
#[test]
fn dylib_2_test() {
    //dylib_2 test
    let r2 = call_reqwest_test();
    match r2 {
        Ok(num) => {
            println!("call_reqwest_test result is {}", num);
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    }
}

/// 命令行执行时的命令参数：
/// C:/Users/admin/.cargo/bin/cargo.exe run --color=always --package simple_main_project --bin simple_main_project
fn main() {
    println!("hello!");
}