pub fn main_t() {
    //from和into是一对，实现了From trait就会自动反过来实现Into，实现都是调用的str.to_owned，to_string调用的String::from
    //所以背后都是调用的to_owned……
    let s1 = "111";
    let s2: String = s1.into();
    let s3: String = s1.to_owned();
    println!("{} {} {}", s1, s2, s3);

    let i1: i32 = 1.into();
    println!("{}", i1);

    //一般直接使用to_owned()就好，因为很直观合理
    //    &str -> String
    //把数据从栈中复制到堆中，成为自己的数据
}