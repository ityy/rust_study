pub fn main() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
//    let s3 = s1.add(&s2);
//    let s3 = s1 + &s2;
    let s3 = String::from("bbb") + "ddd";
    println!("{}", s3);
}