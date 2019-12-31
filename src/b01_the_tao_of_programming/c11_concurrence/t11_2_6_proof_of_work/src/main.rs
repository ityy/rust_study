//! 利用 channel 模拟工作量证明
//! 工作量证明（Proof-of-Work，PoW）是一种对应服务与资源滥用、或是阻断服务攻击的经济对策。
//! 一般是要求用户进行一些耗时适当的复杂运算，并且答案能被服务方快速验算，以此耗用的时间、设备与能源做为担保成本，以确保服务与资源是被真正的需求所使用。
//! 此一概念最早由Cynthia Dwork和Moni Naor于1993年的学术论文提出[1]，而工作量证明一词则是在1999年由Markus Jakobsson与Ari Juels.[2]所发表。
//! 现时此一技术成为了加密货币的主流共识机制之一，如比特币所采用的技术。
use std::{
    sync::{Arc, mpsc},
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use crypto::{
    digest::Digest,
    sha2::Sha256,
};
use itertools::Itertools;

/// 本项目要求，根据一个基数（如42），启动若干线程（如8），计算出一个数字，使得这个数字与42的积的hash值以若干个0（如5个）开头。
/// 0的个数决定难度。
/// 使用暴力运算，从1开始尝试，直到得出要求的结果后返回。
const BASE: usize = 42;
const THREAD_NUMBER: usize = 4;
static DIFFICULTY: &'static str = "00000";

/// 元组结构体
struct Solution(usize, String);

/// 验证给定的数字是否满足条件，是则返回解决方案，不是则返回None
fn verify(number: usize) -> Option<Solution> {
    // 新建哈希器
    let mut hasher = Sha256::new();
    // 输入参数
    hasher.input_str(&(number * BASE).to_string());
    // 获得哈希值
    let hash: String = hasher.result_str();
    // 验证是否满足条件
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

/// 以给定数字开始，以线程数为步进，以共享bool作为判断停止依据，以channel发送结果。
/// 做到多线程并发find。
fn find(start_at: usize, sender: mpsc::Sender<Solution>, is_solution_found: Arc<AtomicBool>) {
    for number in (start_at..).step(THREAD_NUMBER) {
        //如果其它线程已找到，结束本线程
        if is_solution_found.load(Ordering::Relaxed) {
            return;
        }
        //如果找到，修改共享bool值，发送结果，结束本线程
        if let Some(solution) = verify(number) {
            is_solution_found.store(true, Ordering::Relaxed);
            sender.send(solution).unwrap();
            return;
        }
    }
}

fn main() {
    println!("PoW demand:");
    println!("\tFind a number, SHA256(the number  * {}) == \"{}......\" ", BASE, DIFFICULTY);
    // 创建共享变量bool
    let is_solution_found = Arc::new(AtomicBool::new(false));
    // 创建一对收发器
    let (sender, receiver) = mpsc::channel();
    println!("Started {} threads", THREAD_NUMBER);
    // 启动若干线程
    for i in 0..THREAD_NUMBER {
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            find(i, sender_n, is_solution_found);
        });
    }
    println!("Please wait...  ");

    // 接收一次即可
    match receiver.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution: ");
            println!("\tThe number is: {}", i);
            println!("\tthe hash result is : {}.", hash);
        }
        Err(_) => panic!("Worker threads disconnected!"),
    }
}
