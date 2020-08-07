//! # 并发写内存原子性测试
//! 并发写一个变量x是否具有原子性，即是否会出现一个线程写了x的低几位，一个线程写了x的高几位的情况?
//!
//! x类型为u8，u64或u128。
//!
//! ## 测试实例：
//! 设有100个合法数值被100个线程循环写入变量x，另有一个线程读取变量x，以测试x是否会出现非法数值(损坏的值)。
//!
//! ## 测试结论：
//! u8-u64的并发写入不会出现损坏，u128的并发写入会出现损坏。

use std::fmt::Debug;
use std::ops::{Mul, Range};
use std::thread;
use std::time::Duration;

use crate::memory_and_pointer_test::memory_operation::MemoryOperation;

// #[test]
pub fn test() {
    // 目标放堆上
    let target_box = Box::new(0u8);
    // let target_box = Box::new(0u64);
    // let target_box = Box::new(0u128);

    // 初始化100个测试数字
    let nums = get_nums::<u8>(8, true);
    // let nums = get_nums::<u64>(64, true);
    // let nums = get_nums::<u128>(128, true);

    // 创建100个线程，每个线程向目标上循环写nums中的1个数字
    create_concurrent_write_threads(&target_box, &nums);
    thread::sleep(Duration::from_secs(10));

    // 检测是否出现损坏的值
    let target_ptr = get_box_inside_ptr(&target_box);
    let mut target;
    unsafe {
        loop {
            target = *target_ptr;
            println!("{}", target);
            if !nums.contains(&target) {
                println!("出现损坏的值:{}", target);
                break;
            }
        }
    }
}

/// # 获取Box内存储的堆上地址
fn get_box_inside_ptr<T>(target_box: &Box<T>) -> *mut T {
    // 获取Box内存储的堆上地址
    let box_addr_num = MemoryOperation::convert_addr_to_int(target_box);
    let box_ptr_usize = box_addr_num as *const usize;
    let target_ptr = unsafe {
        *box_ptr_usize as *mut T
    };
    target_ptr
}

/// # 初始化测试的数字集合
/// - From<u8> 约束是为了让u8能转换为T
/// - std::ops::Mul<Output=T> 约束是为了T与T之间可以相乘。
fn get_nums<T: Copy + Debug + From<u8> + std::ops::Mul<Output=T>>(bit_wide: u32, print_values: bool) -> Vec<T> {
    let mut nums: Vec<T> = vec![];

    // 让数值均匀分布
    for i in 0..100u8 {
        let mut x: T = (i * 2).into();
        let weight = x;
        for j in 1..(bit_wide / 8) {
            x = x * weight;
        }
        nums.push(x);
    }

    if print_values {
        println!("{:?}", nums);
    }

    nums
}

/// # 创建并发写线程
/// Send + 'static 的约束是因为 value 在线程中被使用。
fn create_concurrent_write_threads<T: Copy + Send + 'static>(target_box: &Box<T>, nums: &Vec<T>) {
    // 获取目标在堆上的地址
    let target_ptr = get_box_inside_ptr(&target_box);
    let target_ptr_num = target_ptr as usize;

    let mut threads = vec![];
    for id in 0..100 {
        let value = *nums.get(id).unwrap();
        let child_thread = thread::spawn(move || unsafe {
            let target_ptr = target_ptr_num as *mut T;
            loop {
                *target_ptr = value;
            }
        });
        threads.push(child_thread);
    }
}
