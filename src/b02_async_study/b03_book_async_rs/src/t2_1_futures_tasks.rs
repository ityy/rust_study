//! 异步编程，主要是异步IO编程。因为IO是对另一方行为的等待，如读硬盘，如等待网络返回等。
//! 传统调用方式，是阻塞执行线程并等待IO结果，我们知道操作系统创建线程是有资源消耗的，这在并发量小的时候可以忽略，但在巨大并发面前，阻塞任何一个线程都变得非常昂贵。
//! 我们不得不思考一种新的方式，在当前线程等待IO期间，如何将其复用起来，以避免操作系统创建过多的线程？
//! 我们将体验一种全新的调用函数理念，将引入过去和未来的概念。
//! 传统函数调用，我们将其称为过去，因为其等待过去已完成的值的返回。我们也称为阻塞方法。
//! 新型的函数调用，我们将其称为未来，因为其不再等待，直接返回未来才完成的值。我们也成为异步方法。异步的目标是降低线程开销。
use std::{io as stdio, thread};
use std::fs::File as stdFile;
use std::io::prelude::*;
use std::io::Read;
use std::result::Result::Ok;
use std::time::Duration;

use async_std::{fs::File, io, prelude::*, task};

/// 传统函数：
/// 读文件并返回内容。其中read_to_string()时线程被阻塞停用，等待操作系统完成文件内容的读取后，线程被启用。
/// 我们要做的就是想办法榨取线程停用的这一段小小的时间，来提高线程的利用率。这样可以在不需要巨量线程的情况下，达到提升性能的目的。
fn read_file(path: &str) -> stdio::Result<String> {
    let mut file = stdFile::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[test]
fn test_read_file() {
    let s = read_file("./Cargo.toml").unwrap();
    println!("{}", s);
}

/// 异步函数： 我们了解一下未来的抽象 Future:
/// ```
/// trait Future {
///     //类型别名
///     type Output;
///     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
/// }
/// ```
///仔细看看，我们可以看到以下几点:
///它在输出上是通用的。
///它提供了一个名为poll的函数，允许我们询问当前计算的状态。
///(现在先忽略Pin和上下文，高级别的理解不需要它们。)
/// 每次调用poll()都可能导致以下两种情况之一:
///     计算完成后，poll将返回poll::Ready
///     计算尚未完成执行，它将返回Poll::Pending
///
///Rust现在有一个特殊的语法:async。上面的例子，用async-std实现，看起来像下方这样。
///差别小得惊人，对吧?我们所做的就是标记函数async并插入两个特殊的命令:.await。
///这个async函数设置了一个延迟计算。
/// 当这个函数被调用时，它将产生一个Future<Output = io::Result<String>>，而不是立即返回一个io::Result<String>。
/// (或者，更准确地说，为您生成一个实现Future<Output = io::Result<String>>的类型。)
/// 我们将read_file改造为异步函数：
async fn read_file_async(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    // 在这里，代码将等待Future产生它的值。await并不会阻塞执行线程，而是会被加入到事件循环，等待事件完成后再次被线程轮询到后继续执行。
    // 打开文件的操作完成时，它将返回到这一点。
    // 这就是为什么这种编程风格也称为事件编程的原因。我们正在等待事情 发生
    // 我们等待事情发生(比如打开一个文件)，然后做出反应(开始阅读)。
    // 响应式编程，我们触发事件后去做其他事情，并等待事件发生后来打断我们。这期间我们没有闲着。哪个事件好了我们就响应哪个事件。
    // 这就像一个人在工作，有电话来了就接电话，没有电话则编写文档，水烧开了则去倒开水... 异步，响应式编程，单线程并发，完全是对此的简单模仿。
    // 当同时执行2个或更多这样的函数时，我们的运行时系统就能够用处理当前发生的所有其他事件来填充等待时间。
    // 而不必阻塞代码完成一个之后再去完成下一个了。
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}


/// Tasks （编程语言层面的Thread)
/// task 即任务, 或称携程，用户态线程，是编程语言模拟的线程, 一个线程多个任务, 就像一个进程多个线程一样。
/// 我们将传统函数新建一个系统线程，丢到新系统线程内去运行。
/// 我们的异步函数不需要新建系统线程，我们新建一个用户态线程，将其丢到新的用户态线程内去运行。
/// 既然我们知道Future是什么，我们就可以运行它们！
/// 在async-std中，tasks模块负责此运行。最简单的方法是使用block_on函数：
#[test]
fn test_read_file_async() {
    //调用异步函数必须在异步块内 异步块返回一个Future
    let future = async {
        let result = read_file_async("./Cargo.toml").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e)
        }
    };

    // task::spawn生成并立即执行一个新的task。其需要一个Future, 返回一个JoinHandle。
    // 与thread::spawn一模一样:生成并立即执行一个新的thread，取需要一个闭包，返回一个JoinHandle。
    // JoinHandle 是一个操作句柄, 表示代码已经在运行
    // Rust中的Future有时也称为cold Futures。您需要一些可以开始运行它们的东西。
    // 要运行Future，可能需要一些额外的簿记，例如它是正在运行还是已完成，它在内存中的位置以及当前状态是什么。该簿记部分在“Task”中被抽象。
    // Task是编程语言对系统Thread的模仿，但有一些细微的差别：Task是由程序而不是操作系统内核调度的，并且如果遇到需要等待的地方，则程序本身负责再次唤醒它。
    // 我们稍后再讨论。就像线程一样，async_std任务也可以具有名称和ID。
    // 到目前为止，只要知道已经生成了一个任务，它将继续在后台运行就足够了。
    let reader_task = task::spawn(future);
    println!("Started task!");//此处和task内的打印 先后顺序不定。
    thread::sleep(Duration::from_secs(5));//打印了result的文件内容 程序等待10s
    // 与threads的join函数非常相似，我们现在可以在handle上调用block_on来阻塞当前线程以等待这个task的完成。
    task::block_on(reader_task);
    println!("Stopped task!");//此处最后打印
}


/// Tasks in async_std
/// async_std中的Tasks是核心抽象之一, 就像Rust的线程一样，它们在原始概念上提供了一些实用的功能。
/// Tasks与runtime有关系，但是它们本身是独立的。 async_std Tasks具有许多理想的属性：
/// 1 它们被分配到同一个分配中
/// 2 所有任务都有一个反向通道，使它们可以通过JoinHandle将结果和错误传播到生成任务
/// 3 它们带有有用的元数据用于调试
/// 4 他们支持任务本地存储
fn nothing2() {}

/// Blocking 阻塞
/// 假定任务可以并发运行，可能通过共享同一个执行线程来执行。
/// 这意味着阻塞系统线程的操作（例如Rust的std库中的thread::sleep或io函数）将停止执行共享此线程的所有任务。
/// 其他库（例如数据库驱动程序）具有类似的行为。请注意，阻塞当前线程本身并不是不良行为，只是与async-std的并发执行模型不能很好地融合在一起。
/// 本质上，永远不要在异步块内写阻塞代码：
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
