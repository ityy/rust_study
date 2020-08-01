//! 通用概念
//! 并发（concurrence）
//!     同时应对多件事情的能力。
//!     一个执行单元，依次执行A B C三个任务，且将任务分解，按A1 B1 C1 A2 B2 C2顺序执行。
//!     看起来像同时执行了A B C三个任务，实际上是时分复用。广义上的并发包含并行。
//!     并发的重点是任务如何分解。
//!     并发正在演变为一种新的编程范式，受到越来越多的编程语言支持。并发会变得像当年多线程进入教材一样，成为新的计算机学子的课堂知识。
//! 并行（parallelism）
//!     同时执行多件事情的能力。
//!     三个执行单元，分别执行A B C三个任务。
//!     看起来和实际上都是同时执行了A B C三个任务。
//!
//! 多进程：
//!     进程是资源分配的最小单元。
//!     多进程并发模式如Master-Worker模式，进程之间用Socket通信。
//!     优点：
//!         健壮性
//!     缺点：
//!         进程相对比较重，占用系统资源较多。
//! 多线程：
//!     线程是程序执行的最小单元。
//!     线程是进程内的细分，共享进程的资源。一个进程至少有一个主线程。
//!     优点：
//!         相对比较轻，占用系统资源较少。
//!     缺点：
//!         编程复杂（线程安全，资源竞争），调试困难。
//! 协程：
//!     协程是在单线程情况下使用分时复用策略，执行多个任务。
//!     协程仿佛是线程的线程，也称为用户态线程。
//!     优点：
//!         最轻，占用系统资源最少。
//!     缺点：
//!         无法利用多核。
//!     将协程和多线程配合使用，可以充分利用多核，且又能异步执行，从而支持巨量并发请求。但是从单线程迁移到多线程，除了带来好处之外，也会带来更多的风险。
//!
//! 线程安全：
//!     由于线程的调度完全由操作系统的内核来控制，完全随机不可预料，这就会导致多线程编程产生很多问题。
//!     多线程存在的问题主要是因为资源共享导致。实际上，只有当一个或多个线程对这些资源进行写操作才会出现问题，如果只读不写，不会存在安全问题。
//!     多线程处理共享资源时，访问资源的顺序无法预料，所以写操作是非常危险的。
//!     竟态条件，临界区，数据竞争。
//!     同步，互斥，原子类型。
//!
//!