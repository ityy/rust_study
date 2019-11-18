//! Tasks
//! 既然我们知道Future是什么，我们就可以运行它们！
//! 在async-std中，任务模块负责此运行。最简单的方法是使用block_on函数：



use std::thread;
use std::time::Duration;

use async_std::{fs::File, io, prelude::*, task};

///异步下的File::open
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = async_std::fs::File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

///执行示例
#[test]
fn test1() {
    let reader_task = task::spawn(async {//调用异步函数必须在异步块内 异步块返回一个Future
        let result = read_file("D:\\yang_rust\\rust_study\\Cargo.toml").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e)
        }
    }); //task::spawn 和thread很类似, 用程序的办法实现线程的功能. 返回一个JoinHandle。
    println!("Started task!");//打印结果: 此处先打印
    thread::sleep(Duration::from_secs(10));//打印了result的文件内容 程序等待10s
    task::block_on(reader_task); //类似thread的join 等待task执行完毕join回来
    println!("Stopped task!");//此处最后打印 表明task::spawn后已开始执行
}

/// 一步一步分解说明:
/// ```
/// async {
///     let result = read_file("data.csv").await;
///     match result {
///         Ok(s) => println!("{}", s),
///         Err(e) => println!("Error reading file: {:?}", e)
///     }
/// };
/// ```
/// 上述代码:
/// 这是一个异步块。异步块是调用异步函数所必需的，它将指示编译器包括所有相关的指令。
/// 在Rust中，所有块都返回一个值，异步块碰巧返回一个Future类型的值。
///
/// ```
/// task::spawn(async { });
/// ```
/// 上述代码:
/// spawn需要一个Future, 并开始在Task上运行它, 然后返回一个JoinHandle。
/// Rust中的Future有时也称为cold Futures。您需要一些可以开始运行它们的东西。
/// 要运行Future，可能需要一些额外的簿记，例如它是正在运行还是已完成，它在内存中的位置以及当前状态是什么。该簿记部分在“Task”中被抽象。
/// Task与Thread类似，但有一些细微的差别：Task是由程序而不是操作系统内核调度的，并且如果遇到需要等待的地方，则程序本身负责再次唤醒它。
/// 我们稍后再讨论。就像线程一样，async_std任务也可以具有名称和ID。
/// 到目前为止，只要知道已经生成了一个任务，它将继续在后台运行就足够了。
/// JoinHandle本身就是一个Future，它将在Task完成任务后完成。
/// 与threads和join函数非常相似，我们现在可以在handle上调用block_on来阻止程序（或要调用的线程，具体来说）并等待其完成。
fn nothing1() {}


/// Tasks in async_std
/// async_std中的Tasks是核心抽象之一, 就像Rust的线程一样，它们在原始概念上提供了一些实用的功能。
/// Tasks与runtime有关系，但是它们本身是独立的。 async_std Tasks具有许多理想的属性：
/// 1 它们被分配到同一个分配中
/// 2 所有任务都有一个反向通道，使它们可以通过JoinHandle将结果和错误传播到生成任务
/// 3 它们带有有用的元数据用于调试
/// 4 他们支持任务本地存储
fn nothing2() {}

/// Blocking 阻断
/// 假定任务可以并发运行，可能通过共享同一个执行线程来执行。
/// 这意味着阻止操作系统线程的操作（例如Rust的std库中的std :: thread :: sleep或io函数）将停止执行共享此线程的所有任务。
/// 其他库（例如数据库驱动程序）具有类似的行为。请注意，阻塞当前线程本身并不是不良行为，只是与async-std的并发执行模型不能很好地融合在一起。
/// 本质上，永远不要这样做：
/// ```
/// fn main() {
///     task::block_on(async {
///         // this is std::fs, which blocks
///         // 这是标准库的fs, 会导致线程阻塞
///         std::fs::read_to_string("test_file");
///     })
/// }
/// ```
/// 如果要混合操作种类，请考虑将此类阻塞操作放在单独的线程上。
fn nothing3() {}


/// Errors and panics
/// Tasks 通过正常模式报告错误：如果它们是fallible，则其Output应为Result <T，E>类型。
/// 如果他们是panic，行为会有所不同，具体取决于是否有合理的部分可以解决该panic。如果不是，程序将中止。
/// 实际上，这意味着block_on会将恐慌传播到阻塞组件：
/// ```
/// fn main() {
///     task::block_on(async {
///         panic!("test");
///     });
/// }
/// ```
/// thread 'async-task-driver' panicked at 'test', examples/panic.rs:8:9
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
///
/// panic时，生成的task将中止：
/// ```
/// task::spawn(async {
///     panic!("test");
/// });
///
/// task::block_on(async {
///     task::sleep(Duration::from_millis(10000)).await;
/// })
/// ```
/// thread 'async-task-driver' panicked at 'test', examples/panic.rs:8:9
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
/// Aborted (core dumped)
/// 乍一看似乎很奇怪，但另一种选择是在生成的任务中默默地忽略panic。
/// 当前行为可以通过在生成的任务中panic并对自定义行为做出反应来更改。这使用户可以选择panic处理策略。
fn nothing4() {}

/// 总结
/// async_std带有一个有用的Task类型，该类型与类似于std :: thread的API一起使用。它以结构化和定义的方式涵盖了 error 和 panic 行为。
/// Tasks 是独立的并发单元，有时它们需要进行通信。那就是为什么要引入Stream。
fn nothing5() {}
