//! # Option 枚举 解析
//! 主要用于解决null的问题, 提供一个some,一个none

/// # Option 解值方式1:match 匹配
#[test]
pub fn test() {
    let x = Some(1);
    let y = None;

    let xy = match x {
        Some(x) => x,
        None => 0,
    } + match y {
        Some(y) => y,
        None => 0,
    };

    println!("{}", xy);
}

/// # Option 解值方式2：unwrap() 强制解值
#[test]
fn test2() {
    let x = Some(1);
    let y = x.unwrap();
    assert_eq!(1, y);
}

/// # Option 解值方式3：as_mut() 获取值的可变借用
#[test]
fn test3() {
    let vec = vec![1, 2, 3];
    let mut option = Some(vec);// vec被移动到Some内 vec变量失效
    let vec_mut_ref = option.as_mut().unwrap();
    vec_mut_ref.push(5);
    println!("{:?}", option.unwrap());// 解包option会移出所有权，此时vec_mut_ref和option均失效
}

