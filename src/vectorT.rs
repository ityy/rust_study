/// vec 学习
pub fn mainT() {
    ///新建
    let v1: Vec<i32> = Vec::new(); //new 方式
    let mut v2 = vec![1, 2, 3]; //宏 方式

    ///增加
    v2.push(4);

    ///获取
    let i1 = &v2[1]; // 下标获取
    let i2 = v2.get(2).unwrap(); // get方法获取, 返回option

    ///遍历
    for i in &v2 {
        println!("{}", i);
    }
}