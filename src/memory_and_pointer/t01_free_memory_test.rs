//! 内存释放 与 内存溢出
//! 代码实验

use std::env::var;
use std::thread;
use std::time::Duration;

///测试内存释放
/// 编译为exe后执行观察内存占用
/// 现象: 内存占用保持不变, 维持在几MB。
/// 结论: 内存确实被作用域结束时drop掉了
//#[test]
pub fn drop_test() {
    let old_string = String::from("hello");//new一个String
    let ptr = move_s(old_string);//获取old_string地址,并移出它
    thread::sleep(Duration::from_secs(1));
    loop {
        //无限循环申请内存
        let temp = String::from("顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶");
        println!();
        println!("------after drop-------");
        //打印old_string所在位置的内存变化情况 此时已发生变化
        for i in 0..5 {
            let value = unsafe { *((ptr + i) as *const u8) };
            println!("addr:0x{:X} value:0x{:X}", ptr + i, value);
        };
    }//释放内存
}

//获取s的地址, 并释放掉s
fn move_s(s: String) -> usize {
    let ptr = s.as_ptr() as usize;//获取堆中地址, 转为usize型
    //打印s所在位置的内存变化情况
    for i in 0..5 {
        let value = unsafe { *((ptr + i) as *const u8) };
        println!("addr:0x{:X} value:0x{:X}", ptr + i, value);
    }
    /*
        此时为hello的ascii码:
        addr:0x28AF59A6710 value:0x68
        addr:0x28AF59A6711 value:0x65
        addr:0x28AF59A6712 value:0x6C
        addr:0x28AF59A6713 value:0x6C
        addr:0x28AF59A6714 value:0x6F
    */
    ptr
}//此处drop了s

///不释放内存, 测试内存溢出
/// 编译后运行, 观察内存占用情况.
/// 现象：内存持续不断的增长, 达100MB以上仍在继续增长。
/// 结论: 所有权转移给向量， 内存没有被释放。
pub fn overflow_test() {
    let mut v = vec![];
    let mut i = 0;
    loop {
        //无限循环申请内存
        let temp = String::from("顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶");
        &v.push(temp);//转移所有权
        i += 1;
        println!("count:{}", i);
    }//释放temp, 因其失去了内存的所有权, 这里只是销毁变量, 内存没有被释放
}
