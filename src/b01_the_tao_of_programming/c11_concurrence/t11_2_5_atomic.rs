//! 原子类型 （atomic)
//! 由于锁会影响性能，且有死锁的风险，因此引入了原子类型。
//! 原子类型是编程语言和操作系统的“契约”，基于此契约实现的自带原子操作的类型。原子操作指绝对完整不可打断的操作。
//! 使用原子类型可以实现无锁并发。锁相当于白盒操作，原子类型就是黑盒操作。原子类型有多种操作和需要给定内存顺序参数。
//! Rust已实现的原子类型：
//! Structs
//!     AtomicBool	A boolean type which can be safely shared between threads.
//!     AtomicI8	An integer type which can be safely shared between threads.
//!     AtomicI16	An integer type which can be safely shared between threads.
//!     AtomicI32	An integer type which can be safely shared between threads.
//!     AtomicI64	An integer type which can be safely shared between threads.
//!     AtomicIsize	An integer type which can be safely shared between threads.
//!     AtomicPtr	A raw pointer type which can be safely shared between threads.
//!     AtomicU8	An integer type which can be safely shared between threads.
//!     AtomicU16	An integer type which can be safely shared between threads.
//!     AtomicU32	An integer type which can be safely shared between threads.
//!     AtomicU64	An integer type which can be safely shared between threads.
//!     AtomicUsize	An integer type which can be safely shared between threads.
//! Enums
//!     Ordering	Atomic memory orderings
//! 内存顺序说明：
//!     Rust的内存顺序与C ++ 20相同。 有关更多信息，请参见nomicon。
//!     Relaxed 轻松
//!     没有排序约束，只有原子操作。
//!     对应于c++ 20中的memory_order_relaxed。
//!     No ordering constraints, only atomic operations.
//!     Corresponds to memory_order_relaxed in C++20.
//!
//!     Release 释放
//!     当与存储相结合时，所有先前的操作都将在此值的任何加载之前进行排序，并具有获取(或更强)排序。特别是，所有执行获取(或更强)此值负载的线程都可以看到以前的所有写操作。
//!     请注意，对组合了加载和存储的操作使用这种排序会导致轻松的加载操作!
//!     此顺序仅适用于可执行存储的操作。
//!     对应于c++ 20中的memory_order_release。
//!     When coupled with a store, all previous operations become ordered before any load of this value with Acquire (or stronger) ordering. In particular, all previous writes become visible to all threads that perform an Acquire (or stronger) load of this value.
//!     Notice that using this ordering for an operation that combines loads and stores leads to a Relaxed load operation!
//!     This ordering is only applicable for operations that can perform a store.
//!     Corresponds to memory_order_release in C++20.
//!
//!     Acquire 获得
//!     当与加载相结合时，如果加载的值是由具有释放(或更强)顺序的存储操作写入的，那么所有后续操作都将在该存储之后排序。特别是，所有后续加载将看到在存储之前写入的数据。
//!     请注意，对一个组合了负载和存储的操作使用这种排序会导致轻松的存储操作!
//!     此顺序仅适用于可以执行负载的操作。
//!     对应于c++ 20中的memory_order_acquire。
//!     When coupled with a load, if the loaded value was written by a store operation with Release (or stronger) ordering, then all subsequent operations become ordered after that store. In particular, all subsequent loads will see data written before the store.
//!     Notice that using this ordering for an operation that combines loads and stores leads to a Relaxed store operation!
//!     This ordering is only applicable for operations that can perform a load.
//!     Corresponds to memory_order_acquire in C++20.
//!
//!     AcqRel
//!     同时具有获取和释放的效果:对于负载，它使用获取顺序。对于商店，它使用发布顺序。
//!     注意，在compare_and_swap的情况下，该操作可能最终不执行任何存储，因此它只是获得排序。然而，AcqRel永远不会执行轻松访问。
//!     此顺序仅适用于将加载和存储组合在一起的操作。
//!     对应于c++ 20中的memory_order_acq_rel。
//!     Has the effects of both Acquire and Release together: For loads it uses Acquire ordering. For stores it uses the Release ordering.
//!     Notice that in the case of compare_and_swap, it is possible that the operation ends up not performing any store and hence it has just Acquire ordering. However, AcqRel will never perform Relaxed accesses.
//!     This ordering is only applicable for operations that combine both loads and stores.
//!     Corresponds to memory_order_acq_rel in C++20.
//!
//!     SeqCst 通常推荐这种顺序，需要性能优化的情况下再根据硬件环境选择一种顺序。
//!     例如获取/释放/AcqRel(分别用于加载、存储和加载与存储操作)，附加的保证是所有线程都以相同的顺序看到所有顺序一致的操作。
//!     对应于c++ 20中的memory_order_seq_cst。
//!     Like Acquire/Release/AcqRel (for load, store, and load-with-store operations, respectively) with the additional guarantee that all threads see all sequentially consistent operations in the same order.
//!     Corresponds to memory_order_seq_cst in C++20.


use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// 使用原子类型实现自旋锁
/// 自旋锁就是循环申请判断是否获取锁，没有则继续申请（自旋一词的由来），有则向下执行
#[test]
fn spin_lock() {
    // 原子类型和锁一样，不提供多线程中共享的方法，需要Arc引用计数来实现。
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move || {
        // 写值
        spinlock_clone.store(0, Ordering::SeqCst);
    });
    // 自旋渎值
    while spinlock.load(Ordering::SeqCst) != 0 {}
    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
    println!("end");
}

