//! 锁：线程同步的惯用策略
//! 要解决上一节末尾的问题，我们引入Mutex互斥锁

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::current;

///使用Arc与Mutex 跨多个线程测试：
#[test]
fn test_send_arc() {
    // Arc默认不可变。内部可变性Cell和RefCell跨线程不能用，我们用Mutex来实现内部可变性。
    let s = Arc::new(Mutex::new("Hello".to_string()));

    let mut v = vec![];
    for id in 0..3 {
        let s_clone = s.clone();
        v.push(thread::spawn(move || {
            /// Mutex<T>互斥锁
            /// 如果T实现了Send，那么Mutex<T>就会自动实现Send和Sync。
            /// 可以调用lock()方法阻塞线程，其返回的LockResult<T> 是std::sync模块中定义的错误类型。
            /// MutexGuard<T>是基于RAII机制实现，只要超出作用域范围就会自动释放锁。MutexGuard和Box一样，实现了自动解引用Deref和DerefMut。
            /// 可以调用try_lock()方法尝试获取锁，当获取不到时返回Err，而不是阻塞线程。
            let mut s_mut = s_clone.lock().unwrap();// 获取锁，不能获取则等待锁
            // 由于Mutex的存在，数据竞争被消除。但竟态条件依然存在，如push_str的顺序仍是随机的。但这里不影响。
            s_mut.push_str(format!(" Rust!({})", id).as_str());
        }));
    }
    for thread in v {
        thread.join().unwrap();
    }
    println!("{}", s.lock().unwrap());
}


/// 跨线程恐慌和错误处理
/// 恐慌不会自动在线程间传播。
/// 传播恐慌到父线程
///     当线程发生错误时，JoinHandle的join方法会返回Result<T>，当子线程内部发生恐慌时，该方法会返回Err。通常直接使用unwrap方法，将恐慌传播给父线程。
/// 中毒（Poison）
///     当线程获得锁之后发生恐慌，这种情况称为中毒（Poison）
#[test]
fn test_poison() {
    let mutex = Arc::new(Mutex::new(1));
    let mutex_clone = mutex.clone();
    let _result = thread::spawn(move || {
        let mut data = mutex_clone.lock().unwrap();
        *data = 2;
        panic!("oh no!!!");
    }).join();
    // 测试中毒状态
    assert_eq!(mutex.is_poisoned(), true);

    ///lock()方法返回的LockResult主要用于返回正常或中毒错误。
    match mutex.lock() {
        Ok(_) => unreachable!(),
        Err(e) => {
            // PoisonError包含get_ref和get_mut方法，用于获取被锁包装的T类型。
            let data = e.get_ref();
            println!("recovered:{}", data);
        }
    };
}


/// 死锁
/// 模拟死锁项目：
///     采用8个线程，每个线程模拟掷硬币的场景，规定连续10次掷出正面为一轮。
///     要求每个线程模拟一轮，统计每个线程的掷硬币次数，以及8个线程的平均掷硬币次数。
/// 不会死锁
#[test]
fn test_not_deadlock() {
    let total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;
    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            flip_simulate(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }
    loop {
        let completed = completed.lock().unwrap();
        if *completed == runs {
            let total_flips = total_flips.lock().unwrap();
            println!("Final average: {}", *total_flips / *completed);
            break;
        }
    }
}

fn flip_simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
    let mut continue_positive = 0;
    let mut iter_counts = 0;
    while continue_positive <= target_flips {
        iter_counts += 1;
        let pro_or_con = rand::random();
        if pro_or_con {
            continue_positive += 1;
        } else {
            continue_positive = 0;
        }
    }
    println!("iter_counts: {}", iter_counts);
    let mut total_flips = total_flips.lock().unwrap();
    *total_flips += iter_counts;
}


/// 会死锁
#[test]
fn test_deadlock() {
    let total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;
    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            flip_simulate2(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }
    loop {
        let completed = completed.lock().unwrap();
        while *completed < runs {}
        let total_flips = total_flips.lock().unwrap();
        println!("Final average: {}", *total_flips / *completed);
    }
}

fn flip_simulate2(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
    let mut continue_positive = 0;
    let mut iter_counts = 0;
    while continue_positive <= target_flips {
        iter_counts += 1;
        let pro_or_con = rand::random();
        if pro_or_con {
            continue_positive += 1;
        } else {
            continue_positive = 0;
        }
    }
    println!("iter_counts: {}", iter_counts);
    let mut total_flips = total_flips.lock().unwrap();
    *total_flips += iter_counts;
}