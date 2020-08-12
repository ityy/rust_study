//! # Option<T> 枚举
//! Option是一个枚举，有两个值：
//! -   Some<T>
//! -   None
//!
//! 主要用于解决null的问题：处理有和无的情况，基本上消除了空指针的问题。
//! Rust为Option<T>实现了一套组合子方法，可以用于快捷操作。

/// # Option 取值方式1：match 匹配
#[test]
pub fn test_match() {
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

/// # Option 取值方式2：unwrap() 强制取值
#[test]
fn test_unwrap() {
    let x = Some(1);
    let y = x.unwrap();// 有值则返回，无值则报错
    assert_eq!(1, y);
}

/// # Option 取值的可变借用：as_mut(
#[test]
fn test_as_mut() {
    let vec = vec![1, 2, 3];
    let mut option = Some(vec);// vec被移动到Some内 vec变量失效
    let vec_mut_ref = option.as_mut().unwrap();
    vec_mut_ref.push(5);
    println!("{:?}", option.unwrap());// 解包option会移出所有权，此时vec_mut_ref和option均失效
}


/// # 实例演示：查找最短名字
/// 结果有找到和找不到两种，可以用Option实现
fn get_shortest_name(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
        let mut shortest_name = names[0];
        for name in names.iter() {
            if name.len() < shortest_name.len() {
                shortest_name = *name;
            }
        }
        Some(shortest_name)
    } else {
        None
    }
}

/// # 测试_查找最短名字_Option的普通处理：
#[test]
fn test_get_shortest_name() {
    /// ## 展示最短名字：match 匹配
    fn show_shortest_name_by_match(names: Vec<&str>) -> &str {
        match get_shortest_name(names) {
            Some(n) => n,
            None => "Not Found",
        }
    }


    /// ## 展示最短名字：unwrap系列方法
    fn show_shortest_name_by_unwrap(names: Vec<&str>) -> &str {
        // 直接返回Option内的值，如若没有则panic
        // get_shortest_name(names).unwrap()

        // 直接返回Option内的值，如若没有则返回or给定的值
        get_shortest_name(names).unwrap_or("Not Found")

        // 直接返回Option内的值，如若没有则返回else给定的闭包的值
        // get_shortest_name(names).unwrap_or_else(|| "Not Found")

        // 直接返回Option内的值，如若没有则以给定的信息作为message进行panic
        // get_shortest_name(names).expect("Not Found")
    }

    let names = vec!["wang", "yang", "liu", "zhang"];
    let result = show_shortest_name_by_unwrap(names);
    println!("{}", result);
}


/// # 测试_查找最短名字_Option的高效处理：组合子（Combinator）系列方法
/// 组合子（Combinator）系列方法:
/// -   map(Fn)         map会自动包装为Option<T>，闭包内返回T即可。
/// -   and_then(Fn)    and_then不会自动包装为Option<T>，适合闭包内返回的就是Option<T>的情况。
#[test]
fn test_combinator() {
    /// ## map() 演示
    /// 将获取最短名字 -> 获取最短名字的长度
    /// - 通过 map() 可以无需取出Option的值就在Option内部进行计算。
    /// - 由于 map() 返回的还是Option，可以进行链式调用。
    ///
    /// get_shortest_name_length_by_map 等效于：
    /// ```
    /// fn get_shortest_name_length_by_match(names: Vec<&str>) -> Option<usize> {
    ///    match get_shortest_name(names) {
    ///        Some(name) => Some(name.len()),
    ///        None => None,
    ///    }
    /// }
    /// ```
    fn get_shortest_name_length_by_map(names: Vec<&str>) -> Option<usize> {
        get_shortest_name(names).map(|name| name.len())
    }

    /// ## and_then() 演示
    fn test_and_then() {
        let number = 20_f64;
        let result = Some(number)
            .map(inverse) //-20
            .map(double) //-40
            .map(inverse) //40
            .and_then(log) //5.321928..
            .map(square) //平方
            .and_then(sqrt) //开根
            .unwrap_or(-1.); //提取
        println!("{}", result);
    }


    // 双倍
    fn double(value: f64) -> f64 {
        value * 2. // 浮点数的简略写法
    }

    // 平方
    fn square(value: f64) -> f64 {
        value.powi(2)
    }

    // 取反
    fn inverse(value: f64) -> f64 {
        value * -1.
    }

    // 取log
    fn log(value: f64) -> Option<f64> {
        match value.log2() {
            x if x.is_normal() => Some(x),
            _ => None,
        }
    }

    // sqrt 开平方根
    fn sqrt(value: f64) -> Option<f64> {
        match value.sqrt() {
            x if x.is_normal() => Some(x),
            _ => None,
        }
    }
}
