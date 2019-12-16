//! Option<T>
//! Option是一个枚举，有两个值：
//!     Some<T>
//!     None
//! 用于处理有和无的情况，基本上消除了空指针的问题。


/// 创建Option
/// 查找最短名字
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

/// 演示Option
#[test]
fn test() {
    /// 展示最短名字
    /// match匹配 处理Option
    fn show_shortest_name_by_match(names: Vec<&str>) -> &str {
        match get_shortest_name(names) {
            Some(n) => n,
            None => "Not Found",
        }
    }

    /// 展示最短名字
    /// unwrap系列方法 用于快速处理Option
    fn show_shortest_name_by_unwrap(names: Vec<&str>) -> &str {
        // get_shortest_name(names).unwrap() //直接返回Option内的值，如若没有则panic
        get_shortest_name(names).unwrap_or("Not Found") //直接返回Option内的值，如若没有则返回or给定的值
        // get_shortest_name(names).unwrap_or_else(|| "Not Found") //直接返回Option内的值，如若没有则返回else给定的闭包的值
        // get_shortest_name(names).expect("Not Found") //直接返回Option内的值，如若没有则以给定的信息作为message进行panic
    }


    //测试
    let names = vec!["wang", "yang", "liu", "zhang"];
    let result = show_shortest_name_by_unwrap(names);
    println!("{}", result);
}


/// 高效处理Option中的值
/// 组合子（Combinator）系列方法:
///     map(Fn)         map会自动包装为Option<T>，闭包内返回T即可。
///     and_then(Fn)    and_then不会自动包装为Option<T>，适合闭包内返回的就是Option<T>的情况。
#[test]
fn test2() {
    /// 获取最短名字的长度
    /// match匹配 处理Option
    fn get_shortest_name_length_by_match(names: Vec<&str>) -> Option<usize> {
        match get_shortest_name(names) {
            Some(name) => Some(name.len()),
            None => None,
        }
    }

    /// 获取最短名字的长度
    /// map() 快速操作Option内的值，仍以Option作为返回值。
    /// 通过map()方法就可以无需取出Option的值，方便在Option内部进行计算。
    /// 由于map()返回的还是Option，这就可以进行链式调用了。
    /// 像map()这样的方法，被称为组合子（Combinator）。
    fn get_shortest_name_length_by_map(names: Vec<&str>) -> Option<usize> {
        get_shortest_name(names).map(|name| name.len())
    }
}

#[test]
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