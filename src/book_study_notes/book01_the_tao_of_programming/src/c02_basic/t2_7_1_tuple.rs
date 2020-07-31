//! # 元组（Tuple）
//! 元组是一种异构有限序列，形如（T,U,M,N）。<br/>
//! 异构，指元组内的元素可以是不同类型的。<br/>
//! 有限，指元组有固定长度。<br/>

/// 元组示例
#[test]
fn test_tuple() {
    // 声明元组
    let tuple = ("hello", 1, 1.5);
    println!("{:#?}", tuple);
    /*打印结果
        ("hello", 1, 1.5)
    */

    // 元组作为函数参数和返回值
    fn move_coords(location: (i32, i32)) -> (i32, i32) {
        //通过索引访问元组，从0开始
        (location.0 + 1, location.1 + 1)
    }
    let coords = (1920, 1080);
    let result = move_coords(coords);
    println!("result is {:#?}", result);

    // 解构元组
    let (x, y) = result;
    println!("{} {}", x, y);
}