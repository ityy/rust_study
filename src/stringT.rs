pub fn main() {
    string_slice_test();
}

fn string_test() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
//    let s3 = s1.add(&s2);
//    let s3 = s1 + &s2;
    let s3 = String::from("bbb") + "ddd";
    println!("{}", s3);

    let x = "hello"; //等价下句
    let x: &'static str = "world"; //等价上句 字符串字面量是写死到代码里的,有固定访问地址,是全局变量
}

fn string_slice_test() {
    //原始字符串 存放着字符串的内存地址和长度
    let str_raw: &'static str = "rust 是一门优雅的语言";
    //获取C指针
    let raw_ptr = str_raw.as_ptr();
    //获取长度
    let len = str_raw.len();

    let addr = &str_raw as *const &str as usize;
    println!("raw_ptr：0x{:X}", raw_ptr as usize);

    println!("addr：0x{:X}", addr);

    let x = addr as *mut &str;
    unsafe {
        let n = &**x; //**X拿到了原始数据, 但因为不能move, 必须使用引用, 使用&再生成一个引用地址
        println!("n_addr：{:p}", n);
    }



    //通过指针的方式 获取数据切片
    let s = unsafe {
        //slice为一个字节数组
        let slice = std::slice::from_raw_parts(raw_ptr, len);
        std::str::from_utf8(slice)
    };
    println!("raw_str:{}", s.unwrap());
}