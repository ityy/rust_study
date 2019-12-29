//! 异步项目： 聊天室
//! 没有什么比创建聊天服务器更简单了，对吧？聊天服务器向您展示了异步编程的所有乐趣：
//! 服务器将如何处理客户端并发连接？
//! 如何处理它们断开连接？
//! 它将如何分发消息？
//! 本教程说明了如何在async-std中编写聊天服务器。
//! 你也可以在此仓库找到源码：https://github.com/async-rs/async-std/tree/master/examples/a-chat

/// 规格和入门
/// 聊天使用基于TCP的简单文本协议。该协议包含utf-8消息，以\n分隔
/// 客户端连接到服务器，并作为第一行发送登录信息。之后，客户端可以使用以下语法将消息发送给其他客户端：
/// login1, login2, ... loginN: message
/// 然后，每个指定的客户端都会收到消息：from login: message
/// 如下所示：
/// On Alice's computer:   |   On Bob's computer:
///
/// > alice                |   > bob
/// > bob: hello               < from alice: hello
///                        |   > alice, bob: hi!
///                            < from bob: hi!
/// < from bob: hi!        |
///
/// 聊天服务器的主要挑战是跟踪许多并发连接。聊天客户端的主要挑战是管理并发传出消息，传入消息和用户键入。
fn nothing1() {}

/// Getting Started 开始
/// Let's create a new Cargo project:
/// $ cargo new a-chat
/// $ cd a-chat
///
/// Add the following lines to Cargo.toml:
/// [dependencies]
/// futures = "0.3.1"
/// async-std = "1.0.1"
///
/// 此项目已独立到b03_t3_chatroom
fn nothing2() {}
