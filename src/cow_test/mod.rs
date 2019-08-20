use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    lst.iter().fold(0, |acc, &n| acc + n)
}

pub fn main() {

    //没有可变需求, 不会发生克隆
    let s1 = [1, 2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);

    //有可变需求 会克隆
    //注意:借用数据被克隆为了新的对象
    let s2 = [1, 2, 3, -45, 6];
    let mut i2=Cow::from(&s2[..]);
    abs_all(&mut i2);
    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2);




}