//! 元组（Tuple）
//! 元组是一种异构有限序列，形如（T,U,M,N）。
//! 异构，指元组内的元素可以是不同类型的。
//! 有限，指元组有固定长度。

/// 元组示例
#[test]
fn test_tuple() {
    //移动坐标 参数为元组，返回值为元组
    fn move_coords(location: (i32, i32)) -> (i32, i32) {
        //通过索引访问元组，从0开始
        (location.0 + 1, location.1 + 1)
    }

    let tuple = ("hello", 1, 1.5);
    println!("{:?}", tuple);
    /*打印结果
        ("hello", 1, 1.5)
    */

    let coords = (1920, 1080);
    //解构元组
    let (x, y) = move_coords(coords);
    println!("{} {}", x, y);
}