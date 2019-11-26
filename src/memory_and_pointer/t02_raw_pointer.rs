//! 原生指针学习
//! 一个数据在内存中,有两种表现,一个是数据本身, 一个是数据所在内存地址
//! ----------
//! |  地址   |
//! ----------
//! |  数据   |
//! ----------
//!
//! 一般来说一个变量a, 通常a表示数据, &a表示地址.
//! 一般&操作取到的地址都是栈上的地址,存放真正堆上的地址和一些描述信息. 真正堆上的地址一般用as_ptr获取, 但其会丢失描述信息.
//!
//! 注意：所有变量标识都本身代表一个栈上的内存地址。代码中使用变量标识，默认都是使用这个标识所标识的地址中的值。
//! 栈:
//! let [a]=3; a代表0x00000001 [a]为3
//! 堆:
//! let [b]=String::new("hello"); b代表0x00000002 [b]为0x10000001 此为hello在堆中的地址。
//! 变量是用来储存信息的，所以我们不关心变量标识代表的内存地址（这一块由编译汇编手段来自动完成替换），我们只关心变量标识存储的信息。
//! 所以实际编程中，变量标识直接代表其存储的信息，而不代表一个内存地址。相当于省略了[]地址符号，[a]直接写作a。

#[test]
/// 栈上内存学习
fn place_expression_stack() {
    ///引用地址类型 原生地址类型 整数型地址
    let mut x = 10;//给x赋值10 变量标识x本身代表一个栈上的内存地址，此操作为此地址赋值。
    let mut x_addr = &mut x;//取x所代表的内存地址
    let x_raw_addr = x_addr as *mut i32;//取x的原生地址 将引用地址转为原生地址类型
    let x_addr_value = x_raw_addr as usize;//取x的地址作为普通的整形数值 可以通过将原生地址转换为整数的形式获得。x_addr_value具有极大破坏性，突破了所有权限制，突破了可变性限制。
    println!("变量x的地址 引用类型: {:p}", x_addr);//编程中使用变量标识，都是使用它所存储的值。x_addr中存储着变量x的地址。
    println!("变量x的地址 原生地址类型: {:p}", x_raw_addr);//{:p} 将变量的值按地址格式打印，等于打印指针。
    println!("变量x的地址 整数类型: 0x{:X}", x_addr_value);//{:X} 将整形按16进制格式打印。
    /*打印结果：
        变量x的地址 引用类型: 0x2d633ff154
        变量x的地址 原生地址类型: 0x2d633ff154
        变量x的地址 整数类型: 0x2D633FF154
      可见都是一样的，只是对这些类型使用的限制各有不同。
    */

    ///使用unsafe块 从任意整数型地址取值
    unsafe {
        //从x_addr_value开始, 打印4个字节的内存
        for i in 0..4 {
            let x = (x_addr_value + i) as *mut u8;//将整数型地址 转回原生地址类型 此处要打印字节，所以按u8类型处理
            println!("addr:0x{:X} value:0x{:X}", x_addr_value + i, *x);
        }
    }
    /*打印结果：栈上内存直接存放着值, 没有堆上内存地址这一中间信息
        addr:0x2D633FF154 value:0xA
        addr:0x2D633FF155 value:0x0
        addr:0x2D633FF156 value:0x0
        addr:0x2D633FF157 value:0x0
    */

    println!();
    println!("-------temp_addr--------");
    //由于42是字面量，是一个只读值。这里编译器会给&42临时创建一个内存地址
    let temp_addr1 = &42;
    let temp_addr2 = &&42;
    let temp_addr3 = &&&42;
    println!("temp_addr1 变量地址: {:p}", &temp_addr1);
    println!("temp_addr1 变量值: {:p}", temp_addr1);
    println!("temp_addr2 变量地址: {:p}", &temp_addr2);
    println!("temp_addr2 变量值: {:p}", temp_addr2);
    println!("temp_addr3 变量地址: {:p}", &temp_addr3);
    println!("temp_addr3 变量值: {:p}", temp_addr3);
}


/// 堆上内存学习
#[test]
fn place_expression_heap() {
    let mut x = "test".to_string(); //x是一个位置表达式 右边字面量是一个值表达式 "test".to_string()将其放在堆中，并返回一个String类型指针
    //x在栈上, 地址为0xFA4DEFE3B0, 值为字符串在堆上的起始地址,长度等. 所以string是一个字符串类型的指针,
    //addr:0xFA4DEFE3B0 value:0x10
    //addr:0xFA4DEFE3B1 value:0x68
    //addr:0xFA4DEFE3B2 value:0x68
    //addr:0xFA4DEFE3B3 value:0xCC
    //addr:0xFA4DEFE3B4 value:0x9A
    //addr:0xFA4DEFE3B5 value:0x2
    //addr:0xFA4DEFE3B6 value:0x0
    //addr:0xFA4DEFE3B7 value:0x0
    //addr:0xFA4DEFE3B8 value:0x4  //长度4
    println!("string:{}", x);//打印string这个字符串指针
    /*
        凡声明一个变量, 变量符号就表示栈上的一个内存地址. string就是栈上的地址0xFA4DEFE3B0, 此地址存放着字符串的堆上地址以及长度等信息.
        0x29ACC686810此地址为字符串在堆上的真实地址, 值为0x74 (t的acsii编码)
    */
    /*
        rust中的Box,Arc,以及String等类型本质上都是智能指针.
        与c语言不同, c语言没有那么多类型, 指针直接指向栈, 所以c语言实现很多复杂类型需要自行设计.
        这一点rust通过智能指针预先设计好了很多类型.
    */
    println!();
    println!("-------Ptr--------");
    let string_ptr = &x; //取出string的地址 注意不是值,也不是堆上的字符串的地址
    println!("string_ptr 变量地址: {:p}", &string_ptr);
    println!("string_ptr 变量值: {:p}", string_ptr);//将其值按内存地址格式打印
    println!("string_ptr 解一层地址后的值: {}", *string_ptr);//*加不加都可以, println!可以自动解引用来打印值. 只有用{:p}才会打印地址.
    let string_mut_ptr = &mut x;
    println!("string_mut_ptr 变量地址: {:p}", &string_mut_ptr);
    println!("string_mut_ptr 变量值: {:p}", string_mut_ptr);
    println!("string_mut_ptr 解一层地址后的值: {}", *string_mut_ptr);
    let string_raw_ptr = &x as *const String;//字符串类型的原始指针 这里获取不到堆上字符串的地址
    println!("string_raw_ptr 变量地址: {:p}", &string_raw_ptr);
    println!("string_raw_ptr 变量值: {:p}", string_raw_ptr);
    unsafe {
        println!("string_raw_ptr 解一层地址后的值: {}", *string_raw_ptr);
    }

    println!();
    println!("-------打印&string开始的内存 长度10个--------");
    let string_raw_ptr_int = string_raw_ptr as usize;
    println!("string_raw_ptr_int 变量值: 0x{:X}", string_raw_ptr_int);//原始指针和usize可以互相转换, 这就为内存操作提供了无限可能
    //利用string_raw_ptr_int强制打印内存地址与值 彻底看清内存结构.
    unsafe {
        for i in 0..10 {
            let x = (string_raw_ptr_int + i) as *mut u8;
            println!("addr:0x{:X} value:0x{:X}", string_raw_ptr_int + i, *x);
        }
    }

    println!();
    println!("-------as_ptr--------");
    //as_ptr获取的直接是堆上的内存地址0x29ACC686810
    let string_as_ptr = x.as_mut_ptr();
    println!("string_as_ptr 变量地址: {:p}", &string_as_ptr);
    println!("string_as_ptr 变量值: {:p}", string_as_ptr);//0x29ACC686810
    unsafe {
        let ptr_value = *string_as_ptr;//此u8为0x74 即字母t的ascii码  u8是copy型的,x为新变量,所以这里不是移动 是拷贝
        println!("string_as_ptr 解一层地址后的值: {}", *string_as_ptr as char);//0x74
        println!("string_as_ptr 解一层地址再取地址: {:p}", &*string_as_ptr);
        println!("ptr_value 变量地址: {:p}", &ptr_value);
        println!("ptr_value 变量值: 0x{:X} 这是t的ascii编码,已取到最终地址", ptr_value);
        *string_as_ptr = *string_as_ptr + 1;
        println!("string 通过指针修改后: {}", x);
    }


    println!();
    println!("-------Move--------");
    println!("string_raw_ptr_int：0x{:X}", string_raw_ptr_int);
    let string_new = x;//转移所有权 右边是值,左边是一个新位置. Move后地址变化.
    let string_new_addr_int = &string_new as *const String as usize;
    println!("string_new_addr_int：0x{:X}", string_new_addr_int);//以Hex形式打印
    println!("string_new_as_ptr 变量值: {:p}", string_new.as_ptr());//0x29ACC686810 移动后堆上的内存地址是不变的


    println!();
    println!("-------Box--------");
    let string_box = Box::new(string_new);//再次转移所有权 交给指针Box
//    let string_=string_box
    println!("string_box 变量地址: {:p}", &string_box);
    println!("string_box 变量值: {:p}", string_box);//栈上的新指针, 旧指针string, string_new作废
    println!("string_box 解一层地址后的值: {}", *string_box);
    println!("string_box_as_ptr 变量值: {:p}", string_box.as_ptr());//0x29ACC686810 移动后堆上的内存地址是不变的
}
