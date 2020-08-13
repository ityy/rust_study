//! # Result<T, E> 枚举 解析
//! Result<T, E> 是Rust内置的一个枚举，含有Ok(T)、Err(E) 两个元素:
//! ```
//! pub enum Result<T, E> {
//!     /// Contains the success value
//!     #[stable(feature = "rust1", since = "1.0.0")]
//!     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
//!
//!     /// Contains the error value
//!     #[stable(feature = "rust1", since = "1.0.0")]
//!     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
//! }
//! ```
//!
//! 可以将其按普通枚举自由使用：
//! ```
//! // T、E均指定为u32
//! fn err_u32() -> Result<u32, u32> {
//!     // Ok(1)
//!     Err(2)
//! }
//!
//!
//! // T指定为u32，E指定为String
//! fn err_string() -> Result<u32, String> {
//!     // Ok(1)
//!     Err("error message".into())
//! }
//!
//! let result = err_u32().unwrap(); //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: 2', src\main.rs:32:18
//! println!("result:{}", result);
//! let result = err_string().unwrap(); //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "error message"', src\main.rs:34:18
//! println!("result:{}", result);
//! ```
//!
//! # Java 与 Rust 异常处理对比
//! - java将异常与正常返回值进行了维度区分，正常时可以正常返回，异常时则为抛出异常，由处理方使用try-catch进行处理。这种方法显然不够优雅。
//! - Rust将正常和异常统一为返回值这个维度，使用枚举体来解决这两种情况，相关的处理也不会显得割裂，使得语言统一度更高。
//!
//! Rust为Result<T, E>实现了一套组合子方法，可以用于快捷操作。


use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

const FILE_PATH: &'static str = r"C:\study_code\rust_study\src\book_study_notes\book01_the_tao_of_programming\src\c09_panic_handle\test_sum.txt";

/// # Ok或Err的演示
#[test]
fn test_result() {
    // 字符串转数字 正常
    let n = "1";
    assert_eq!(n.parse::<i32>(), Ok(1));

    // 字符串转数字 异常
    let n = "a";
    let result = n.parse::<i32>();
    println!("{:?}", result); // Err(ParseIntError { kind: InvalidDigit })
}

/// # 高效处理 Result<T,E>
/// - Result也拥有unwrap系列方法，用法和Option相似，但并不优雅。
/// - Result也拥有组合子处理方法，用法和Option相似。
#[test]
fn test_combinator() {
    /// # map() 将字符串转为数字，并求平方。
    /// map方法：
    /// - 在正确时，按闭包处理，并将闭包的结果再包装为Result返回。使得看起来像没拆包一样
    /// - 在错误时，直接返回错误。
    fn square(number: &str) -> Result<i32, ParseIntError> {
        number.parse::<i32>().map(|n| n.pow(2))
    }

    match square("123") {
        Ok(n) => println!("number is {}", n),
        Err(e) => println!("Error:{:?}", e),
    };

    /// # 使用类型别名，简化Result<T,E>
    /// 这是Rust中常用的方法，各大库包中都可以看到这种用法。
    /// 如下别名演示，这样并没有起到简化的作用：
    /// ```
    /// type ParseRusult_<T, E> = Result<T, E>;
    /// ```
    /// 我们改造一下，将泛型E直接指明：
    type ParseRusult<T> = Result<T, ParseIntError>;

    /// # 使用类型别名，简化Result<T,E>
    fn square_simplify(number: &str) -> ParseRusult<i32> {
        number.parse::<i32>().map(|n| n.pow(2))
    }

    /// # and_then() 将字符串转为数字，并求平方，再转为字符串返回
    /// and_then方法：
    /// - 在正确时，按闭包处理，并返回闭包的结果（适合闭包返回值也是Result的情况）
    /// - 在错误时，直接返回错误。
    fn square_and_then_to_string(number: &str) -> Result<String, ParseIntError> {
        // fn square_and_then_to_string<T, E>(number: &str) -> Result<T, E> {
        // 由于map和and_then返回的值和错误的类型都不相同，使得函数签名上的Result的泛型T E不知道怎么赋值，这块无法实现。解决方法有二：
        //      1 自定义统一的错误类型，使得其它错误都可以对应到统一错误类型中的某一个错误。IO操作的Error即是如此。
        //      2 使用特性对象，Rust提供了Error trait，Rust中的所有错误都实现了这一特性。
        let x = number
            .parse::<i32>()
            .map(|n| n.pow(2))
            .and_then(|n: i32| Ok(n.to_string()));
        x
    }

    match square_and_then_to_string("1111") {
        Ok(s) => println!("number string is {}", s),
        Err(e) => println!("Error:{:?}", e),
    }
}

/// # 处理不同类型的错误
/// 下方函数展示了一个函数内部多种不同错误的情形。函数功能为从文件读取每行的数字，返回它们的和。
/// 对于Result<T,E>来说，最终只能返回一个E类型，如果在函数中返回了不同的错误类型，编译就会报错。
/// 如何让下方函数可以把不同的错误，传播出来呢？
/// ```
/// fn test_read_error_count() {
///     let mut file = File::open(FILE_PATH).unwrap(); // 读不出文件报错
///     let mut centents = String::new();
///     file.read_to_string(&mut centents).unwrap(); // 内容无法解析报错
///     let mut sum = 0;
///     for c in centents.lines() {
///         sum += c.parse::<i32>().unwrap(); // 内容不能转为数字报错
///     }
///     println!("{:?}", sum);
/// }
/// ```
/// 为了让test_read_error_count()可以返回Result，有两个办法：
/// 1. 自定义统一的错误类型，使得其它错误都可以对应到统一错误类型中的某一个错误。IO操作的Error即是如此。
/// 2. 使用特性对象，Rust提供了Error trait，Rust中的所有错误都实现了这一特性。
///
/// 由于特性对象动态分发的特性，其性能不如自定义统一错误类型，但其方便程度要强于自定义统一错误类型。
///
/// 这里我们使用Error trait，重构test_read_error_count()函数，使用组合子书写业务逻辑：
fn test_read_error_trait(filename: &str) -> Result<i32, Box<dyn Error>> {
    let result = File::open(filename)
        .map_err(
            // map_err和map 操作相同，目标相反：map只处理Ok的值，Err原样返回。map_err只处理Err的值，Ok原样返回。
            |e| e.into(), // into()自动将Err转为Box<dyn Error>类型，从此处开始调用链上传递的错误均为Box<dyn Error>
        )
        .and_then(|mut file| {
            let mut centents = String::new();
            file.read_to_string(&mut centents)
                .map_err(|e| e.into()) // into()自动将Err转为Box<dyn Error>
                .map(|_| centents) // 正确时返回centents 将处理交给下一个调用链。也可直接在这里处理，但代码结构不够优美。
        })
        .and_then(|centents| {
            let mut sum = 0;
            for c in centents.lines() {
                match c.parse::<i32>() {
                    // 只在转换成功时计算
                    Ok(n) => {
                        sum += n;
                    }
                    // 转换失败只打印信息
                    Err(e) => {
                        // 手动转换为Box<dyn Error>
                        let e: Box<dyn Error> = e.into();
                        // 打印dyn Error的信息
                        println!("error info:{},cause:{:?}", e.to_string(), e.source());
                        // 也可以直接中断，将e返回出去
                        // return Err(e);
                    }
                }
            }
            // 返回结果
            Ok(sum)
        });
    result
}

/// # try! 与 操作符?
/// 简化错误处理，直接提取正确结果，如果是错误则向外传播出去。
///
/// try!宏内部对Err调用了from转换，不论是特性对象还是自定义统一错误，都不需要手动调用map_err(|e| e.into())处理错误类型的转换问题了。
///
/// 重构上方的test_read_error_trait()方法，其功能一样，写法更简洁
fn test_read_error_try(filename: &str) -> Result<i32, Box<dyn Error>> {
    // 使用try!宏，直接获取结果。
    // let mut file = try!(File::open(filename).map_err(|e| e.into()));
    // 操作符? 是try!宏的语法糖，使代码更加优雅。
    let mut file: File = File::open(filename)?;
    let mut centents = String::new();
    file.read_to_string(&mut centents)?;
    let mut sum = 0;
    for c in centents.lines() {
        sum += c.parse::<i32>()?; //由于？是遇到错误则直接抛出，有些场景并不适合。
    }
    Ok(sum)
}

/// # 测试结果
#[test]
fn test() {
    let result = test_read_error_trait(FILE_PATH);
//    let result = test_read_error_try(FILEPATH);
    match result {
        Ok(n) => println!("sum is {}", n),
        Err(e) => println!("error info:{},cause:{:?}", e, e.source()),
    }
}


