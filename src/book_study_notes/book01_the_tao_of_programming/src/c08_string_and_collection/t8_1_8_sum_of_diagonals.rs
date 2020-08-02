//! # 字符串测验-对角线之和

#[test]
fn test() {
    //r为raw的简写，r标识符开头的字符为串原生字符串，即以书写时候的样式为准
    let s = r"1234
                    5678
                    9876
                    4321";

    let (mut x, mut y) = (0, 0);
    // enumerate 创建一个迭代器，返回(key,value)，其中key由下标或序号生成，value为对应元素。
    for (idx, val) in s.lines().enumerate() {
        let val = val.trim();
        let left = val[idx..idx + 1].parse::<u32>().unwrap();
        let right = val[(3 - idx)..(3 - idx + 1)].parse::<u32>().unwrap();
        x += left;
        y += right;
    }
    assert_eq!(38, x + y);
}