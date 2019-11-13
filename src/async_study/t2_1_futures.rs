//! 开始
use std::fs::File as stdFile;
use std::io as stdio;
use std::io::prelude::*;
use std::io::Read;
use std::result::Result::Ok;

use async_std::{fs::File, io, prelude::*, task};

///Let's have a look at a simple function, specifically the return value:
///让我们看一个简单的函数
/// 你可以在任何时候调用它，所以你可以完全控制何时调用它。
/// 但问题是:在调用它的那一刻，您将控制权转移到被调用的函数，直到它返回一个值——最终。
/// 注意，这个返回值谈论的是过去。过去有一个缺点:所有的决定都已做出。
/// 它有一个优势:结果是可见的。我们可以打开程序过去计算的结果，然后决定如何处理它。
///
/// 就时间而言，我们只能在调用函数之前或函数返回之后执行操作。
/// 这是不可取的，因为它剥夺了我们在运行时做某事的能力。
/// 当使用并行代码时，这将剥夺我们在第一次运行时启动并行任务的能力(因为我们放弃了控制权)。
fn read_file(path: &str) -> stdio::Result<String> {
    let mut file = stdFile::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


/// ```
/// trait Future {
///     //类型别名
///     type Output;
///     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
/// }
/// ```
///仔细看看，我们可以看到以下几点:
///它在输出上是通用的。
///它提供了一个名为poll的函数，允许我们检查当前计算的状态。
///(现在先忽略Pin和上下文，高级别的理解不需要它们。)
///
/// 每次调用poll()都可能导致以下两种情况之一:
///计算完成后，poll将返回poll::Ready
///计算尚未完成执行，它将返回Poll::Pending
#[test]
fn test() {
    let s = read_file("D:\\yang_rust\\rust_study\\Cargo.toml").unwrap();
    println!("{}", s);
}

///异步的目标是降低线程开销
///Rust现在有一个特殊的语法:async。上面的例子，用async-std实现，看起来像下方这样。
///差别小得惊人，对吧?我们所做的就是标记函数async并插入两个特殊的命令:.await。
///这个async函数设置了一个延迟计算。
/// 当这个函数被调用时，它将产生一个Future<Output = io::Result<String>>，而不是立即返回一个io::Result<String>。
/// (或者，更准确地说，为您生成一个实现Future<Output = io::Result<String>>的类型。)
///
/// 把此方法丢进一个单线程loop内调用
async fn read_file_async(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;//在这里，代码将等待Future产生它的值。 这里被称为parked, 事件完成后会unpark
    //当您在后台执行的操作完成时，它将返回到这一点。
    // 这就是为什么这种编程风格也称为事件编程的原因。我们正在等待事情 发生
    // 我们等待事情发生(比如打开一个文件)，然后做出反应(开始阅读)。
    // 当同时执行2个或更多这样的函数时，我们的运行时系统就能够用处理当前发生的所有其他事件来填充等待时间。
    // 而不必阻塞代码完成一个之后再去完成下一个了。
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
