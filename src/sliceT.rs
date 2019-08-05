///切片测试
///字符串少用切片, 会切出不完整的unicode编码导致报错
pub fn slice_test() {
    let s = String::from("你好啊哈哈哈");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{},{}", hello, world);
}
