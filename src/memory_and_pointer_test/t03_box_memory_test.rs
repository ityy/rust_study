//! # Box 内存结构研究

use std::rc::Rc;

use crate::memory_and_pointer_test::memory_operation::MemoryOperation;

/// # Box 解析
/// 验证 Box::new(目标） 目标会被转移到堆上。
fn box_int() -> (usize, Box<usize>) {
    println!("________test_box_int()________");
    /* x_box 绑定了一个 Box：
     *      x_box 的类型为 Box<T> 类型。
     *      x_box 的值为Box的存储结构。
     * Box的结构为：
     *      栈内存地址        Box对象
     *                  |---------------|
     *      0x0001  ->  |目标的地址      |
     *                  |---------------|
     * &x_box=0x0001
     * x_box本身代表[x_box]语义，即x_box代表变量自身的值：Box对象。
    */
    let x_box = Box::new(42usize);// 以下称42usize为目标。目标被放在堆上，目标地址存在Box结构内。

    let x_box_addr = &x_box;
    println!("局部变量 x_box 的地址（指针）: {:p}", x_box_addr);
    let x_box_addr_num = x_box_addr as *const std::boxed::Box<usize> as usize;
    println!("局部变量 x_box 的地址（数字）: 0x{:X}", x_box_addr_num);

    //打印Box内存结构
    MemoryOperation::print_addr_and_value(x_box_addr_num, 8);

    //取box内的目标地址
    let x_box_addr = x_box_addr_num as *const usize;// 数字形式地址转指针形式地址，添加一个usize类型描述。
    let target_addr_num = unsafe {
        *x_box_addr// 取出usize，即为目标地址的数字形式。
    };
    println!("target_addr_num: 0x{:X}", target_addr_num);

    /*
     * 将目标的实际地址传出，防止Box被回收，也将Box传出。
     *      target_addr_num 被按值复制传出
     *      x_box 将Box结构复制后传出，转移Box所有权
     *
     * 所有局部变量销毁
     */
    (target_addr_num, x_box)
}

/// # “Box 解析” 的测试
#[test]
fn box_int_test() {
    /*
     * 使用本栈区的局部变量接收数字，Box对象
     */
    let (target_addr_num, x_box) = box_int();
    println!("________main()________");

    /*
     * 将数字地址还原为指针地址，按usize类型描述。并通过指针取目标值。
     */
    {
        let target_addr = target_addr_num as *const usize;
        println!("target_addr: {:p}", target_addr);
        unsafe {
            println!("通过指针取target: {}", *target_addr);
            let target_ref = &*target_addr;// 把指针还原为引用演示
        }
        //回收资源
    }

    /*
     * 打印Box结构内容
     */
    let x_box_addr_num = &x_box as *const Box<usize> as usize;
    MemoryOperation::print_addr_and_value(x_box_addr_num, 8);


    /*
     * 从Box取出目标，目标被move，Box作废。
     */
    let target = *x_box;//值复制
    println!("通过Box取target: {}", target);
    drop(x_box);//回收x_box

    /*
     * 因x_box被释放，下面示例则为不安全指针，值可能不正确。
     * 将数字地址还原为指针地址，按usize类型描述。并通过指针取目标值。
     */
    {
        let target_addr = target_addr_num as *const usize;
        println!("target_addr: {:p}", target_addr);
        unsafe {
            println!("通过指针取target: {}", *target_addr);
            let target_ref = &*target_addr;// 把指针还原为引用演示
        }
    }
}


/// # 堆内存地址测试
/// 与栈内存地址测试性质一样。
#[test]
fn heap_memory_box_test() {
    println!("\r\n-------Box智能指针解析--------");
    let x = "test".to_string();
    println!("文本地址: {:p}", x.as_ptr());

    let string_box = Box::new(x);// String智能指针被放入堆内存中，且所有权转移给智能指针Box。x因失去String的所有权而作废。
    println!("string_box 变量值（指针格式）: {:p}", string_box);// String智能指针在堆内存中的地址
    println!("string_box 变量值（自动解引用）: {}", string_box);

    let content_ptr1 = string_box.as_ptr();//相当于(*string_box).as_ptr() Box类型在使用时可以自动解引用
    let content_ptr2 = (*string_box).as_ptr();
    println!("文本地址: {:p}", content_ptr1);//堆上的文本内存地址是不变的
    println!("文本地址: {:p}", content_ptr2);//堆上的文本内存地址是不变的

    println!("\r\n-------Box智能指针 内存结构--------");
    let box_addr_int = MemoryOperation::convert_addr_to_int(&string_box);
    MemoryOperation::print_addr_and_value(box_addr_int, 8);// 8byte: String智能指针在堆内存中的地址

    /*总结：
        增加了Box指针后，原String指针被挪到了堆上，Box指针指向堆上的String指针，String指针再指向堆上的字符串内容。
        等于说是在中间加了一层指针包装。
    */
}