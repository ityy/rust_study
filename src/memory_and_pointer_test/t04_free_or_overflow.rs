//! # 内存释放 与 内存溢出
//! 代码实验

use std::env::var;
use std::thread;
use std::time::Duration;

use crate::memory_and_pointer_test::memory_operation::MemoryOperation;

/// # 内存释放 测试
/// 使用一个简单的loop循环不断的申请内存 <br/>
/// 最好编译为exe后再执行，在任务管理器中观察内存占用 <br/>
/// > 现象: 内存占用保持不变, 维持在几MB。 <br/>
/// > 结论: 内存确实被作用域结束时drop掉了 <br/>
// #[test] 不能使用test测试，请使用main函数测试
pub fn drop_test() {
    let s = String::from("hello");//new一个String
    let text_addr_num = drop_and_get_addr_num(s);//获取old_string地址,并移出它
    loop {
        thread::sleep(Duration::from_secs(1));
        //无限循环申请内存
        let temp = String::from("顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶");
        println!("------after drop-------");
        //打印s中堆地址的内存变化情况，此时已发生变化，说明被drop的内存可以正常被重新利用
        MemoryOperation::print_addr_and_value(text_addr_num, 5);
        /*
         * 此时已发生变化
         * addr:0x2A1EC456B60 value:0x50
         * addr:0x2A1EC456B61 value:0xE9
         * addr:0x2A1EC456B62 value:0x45
         * addr:0x2A1EC456B63 value:0xEC
         * addr:0x2A1EC456B64 value:0xA1
         */
    }//释放内存
}

/// 回收s，但返回s中的字符串堆地址
fn drop_and_get_addr_num(s: String) -> usize {
    let text_addr_num = s.as_ptr() as usize;//获取堆中地址, 转为usize型
    //打印s中堆地址的内存变化情况
    MemoryOperation::print_addr_and_value(text_addr_num, 5);
    /*此时为hello的ascii码:
        addr:0x2A1EC456B60 value:0x68
        addr:0x2A1EC456B61 value:0x65
        addr:0x2A1EC456B62 value:0x6C
        addr:0x2A1EC456B63 value:0x6C
        addr:0x2A1EC456B64 value:0x6F
    */
    text_addr_num
}//此处drop了s

/// # 内存溢出 测试
/// 使用一个简单的loop循环不断申请内存，且使用一个外部容器保存申请的内容。 <br/>
/// 最好编译为exe后再执行，在任务管理器中观察内存占用 <br/>
/// 现象：内存持续不断的增长, 达100MB以上仍在继续增长。 <br/>
/// 结论: 所有权转移给向量， 内存没有被释放。 <br/>
// #[test] 不能使用test测试，请使用main函数测试
pub fn overflow_test() {
    let mut v = vec![];
    let mut i = 0;
    loop {
        //无限循环申请内存
        let temp = String::from("顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶顶");
        &v.push(temp);//转移所有权（复制智能指针到容器内）
        i += 1;
        println!("count:{}", i);
    }//释放temp, 因其已失去所有权，仅销毁了变量temp与其值String智能指针，堆上内存没有变化。
}
