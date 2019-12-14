//! Option 枚举
//! 主要用于解决null的问题, 提供一个some,一个none

/// Option 解值方式1:match匹配
#[test]
pub fn test() {
    let some_number = Some(5);
    let some_string = Some("is a string");
    let absent_number: Option<i32> = None;

    let x = Some(1);
    let y = None;
    let xy = match x {
        Some(x) => x,
        None => 0,
    }
        +
        match y {
            Some(y) => y,
            None => 0,
        };
    println!("{}", xy);
}

/// Option 解值方式2：unwrap()强制解值
#[test]
fn test2() {
    let x = Some(1);
    let y = x.unwrap();
}

/// Option 解值方式3：as_mut() 获取值的可变借用
#[test]
fn test3() {
    let vec = vec![1, 2, 3];
    let mut x = Some(vec);
    // println!("{:?}", vec);// vec被移动到Some内 vec变量作废
    let y = x.as_mut().unwrap();
    y.push(5);
    println!("{:?}", x.unwrap());// as_mut()只是可变借用 x仍有所有权
}

