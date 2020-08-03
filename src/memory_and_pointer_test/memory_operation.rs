//! # 内存操作封装
//! author：yangyang <br/>
//! since：2019年8月21日 <br/>


pub struct MemoryOperation;

impl MemoryOperation {
    /// ## 打印内存
    /// 入参（数字型起始地址，字节数量）<br/>
    /// 原理：遍历数字型地址，将数字型地址转为指针，按u8类型描述。取指针值并打印即可。
    pub fn print_addr_and_value(addr_begin: usize, byte_count: usize) {
        println!("开始内存打印 -> 起始地址：{:X}，字节数：{}", addr_begin, byte_count);
        //需要访问原生指针，则必须在unsafe块中使用
        unsafe {
            for i in 0..byte_count {
                let x = (addr_begin + i) as *const u8;//将整数型地址，转为u8型raw指针（此处要打印字节，所以需要u8型指针）。
                println!("addr:0x{:X} value:0x{:X}", addr_begin + i, *x); //取指针x存储的数据
            }
        }
    }

    /// ## 内存地址格式转换
    /// 将T类型的Rust引用型（安全指针类型，即安全内存地址类型）转为整数类型
    pub fn convert_addr_to_int<T>(t: &T) -> usize {
        // rust引用和raw指针 raw指针和usize 可以互相转换, 这就为内存操作提供了无限可能
        // 指针形式地址转数字形式地址，他俩是相同的内存地址，区别是指针带有类型描述，可以按类型解析。数字形式没有类型描述信息。
        // *const表示原生不可变指针， *mut表示原生可变指针。
        t as *const T as usize
    }


    /// ## 内存地址格式逆转换
    /// 将整数类型转为T类型的Rust引用型（安全指针类型，即安全内存地址类型）
    pub unsafe fn convert_int_to_addr<'a, T>(addr: usize) -> &'a T {
        let raw_ptr = addr as *const T;//1 将整数型地址转为T型原生指针
        &*raw_ptr//2 原生指针转为Rust引用的方法：解指针得到对象，再引用
    }
}