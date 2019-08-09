///切片测试
///字符串少用切片, 会切出不完整的unicode编码导致报错
// #[test]
// 加了test注解可以直接用test运行, 此方法转为测试方法, 外部无法调用
pub fn slice_test() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{},{}", hello, world);
    assert_eq!("hello", hello);
}
