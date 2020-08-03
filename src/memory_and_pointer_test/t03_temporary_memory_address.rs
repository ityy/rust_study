//!  # 编译器生成临时内存地址学习

use crate::memory_and_pointer_test::memory_operation::MemoryOperation;

#[test]
fn temp_memory_addr() {
    //由于42是字面量，是一个只读值。这里编译器会给&42创建一个临时内存地址
    let temp_addr1 = &42;
    let temp_addr2 = &&42;
    let temp_addr3 = &&&42;

    // 打印变量自身地址，3个变量的地址几乎是连续分配的。
    println!("temp_addr1 变量地址: {:p}", &temp_addr1);
    println!("temp_addr2 变量地址: {:p}", &temp_addr2);
    println!("temp_addr3 变量地址: {:p}", &temp_addr3);
    /*打印结果
        temp_addr1 变量地址: 0x48c24fee08
        temp_addr2 变量地址: 0x48c24fee10
        temp_addr3 变量地址: 0x48c24fee18
    */

    // 打印编译器临时生成的地址
    println!("temp_addr1 变量值: {:p}", temp_addr1);
    println!("temp_addr2 变量值: {:p}", temp_addr2);
    println!("temp_addr3 变量值: {:p}", temp_addr3);
    /*打印结果
        temp_addr1 变量值: 0x7ff6ee7601e0
        temp_addr2 变量值: 0x7ff6ee7601e8
        temp_addr3 变量值: 0x7ff6ee7601f0
    */

    // 观察生成地址的链接情况
    let temp_addr3_value = MemoryOperation::convert_addr_to_int(temp_addr3);
    let temp_addr3_ptr = temp_addr3_value as *const usize;
    let temp_addr3_ptr_value = unsafe { *temp_addr3_ptr };
    println!("temp_addr3的值所表示的地址中的值: 0x{:X}", temp_addr3_ptr_value);

    let temp_addr2_value = MemoryOperation::convert_addr_to_int(temp_addr2);
    let temp_addr2_ptr = temp_addr2_value as *const usize;
    let temp_addr2_ptr_value = unsafe { *temp_addr2_ptr };
    println!("temp_addr2的值所表示的地址中的值: 0x{:X}", temp_addr2_ptr_value);

    let temp_addr1_value = MemoryOperation::convert_addr_to_int(temp_addr1);
    let temp_addr1_ptr = temp_addr1_value as *const usize;
    let temp_addr1_ptr_value = unsafe { *temp_addr1_ptr };
    println!("temp_addr1的值所表示的地址中的值: 0x{:X}", temp_addr1_ptr_value);
    /*结论：
        0x7ff6ee7601f0 内 存放者 0x7ff6ee7601e8
        0x7ff6ee7601e8 内 存放者 0x7ff6ee7601e0
        0x7ff6ee7601e0 内 存放者 0x2A （42的十六进制）
     */
}
