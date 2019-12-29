/// 使用异步标准库
use async_std::{
    // 对于socket类型，我们使用async_std中的TcpListener，就像std :: net :: TcpListener一样，但是是非阻塞的，并且使用了异步API。
    net::{TcpListener, ToSocketAddrs},
    // prelude 预导入一些特性, 这些特性是futures和streams所必须的.
    prelude::*,
    // task 模块大致对应于std :: thread模块，但 task 的重量轻得多。一个线程可以运行许多任务。
    task,
};

/// 在此示例中，我们将跳过实现全错误处理。
/// 为了传播错误，我们将使用装箱的Error特性对象。
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// 现在我们可以编写服务器的accept循环：
/// 我们将accept_loop函数标记为async，这允许我们在其中使用.await语法。
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    //TcpListener :: bind调用返回一个future，我们等待提取结果，然后？获取一个TcpListener。注意如何.await和？一起很好地工作。
    //这正是std::net::TcpListener的工作方式，但是添加了.await。 std的镜像API是async_std的明确设计目标。
    let listener = TcpListener::bind(addr).await?;//此处会阻塞task直到功能完成获取返回值, 执行task得线程则不会阻塞, 会继续向下轮询其它task执行. awati得大致含义是:线程你去做别的事情吧, 等我这里好了再过来执行我
    let mut incoming = listener.incoming();
    // 在这里，我们想迭代传入的套接字，就像在std中那样：
    // let listener: std::net::TcpListener = unimplemented!();
    // for stream in listener.incoming() {
    //      //do something
    // }
    // 不幸的是，这还不适用于异步，因为该语言还不支持异步for循环。
    // 因此，我们必须使用while let Some（item）= iter.next（）.await模式来手动实现循环。
    while let Some(stream) = incoming.next().await {
        println!("{:?}", stream);
    }
    Ok(())
}


/// 测试
#[test]
fn run() -> Result<()> {
    // 调用异步函数会返回Future
    // 与其他语言不同，在Rust中要意识到的关键是，调用异步函数不会运行任何代码。主要是看返回值,如果是Future,一定是未运行得代码.如果是handle,一定是正在运行得代码,返回了一个操作句柄
    // 异步功能仅构造Future，它们是惰性状态机。要开始使用异步功能逐步遍历Future的状态机，应使用.await。
    let fut = accept_loop("127.0.0.1:8080");
    // 在非异步函数中，执行Future的一种方法是将其交给执行者(executor)。
    // task::block_on(task)可以用来阻塞当前线程并等待task执行完成。
    // task::block_on(f?uture) 可以不需要spawn一个新的task，而是直接在当前线程上执行Future并阻塞直到完成。
    // fut内是一个循环监听，此处将不会返回
    task::block_on(fut)
}