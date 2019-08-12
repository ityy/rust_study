pub fn main() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
//    let s3 = s1.add(&s2);
//    let s3 = s1 + &s2;
    let s3 = String::from("bbb") + "ddd";
    println!("{}", s3);

    let x = "hello"; //等价下句
    let x: &'static str = "world"; //等价上句 字符串字面量是写死到代码里的,有固定访问地址,是全局变量
}

