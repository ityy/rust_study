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
const THREADS: usize = 4;
static DIFFICULTY: &'static str = "000000";

struct Solution(usize, String);

fn verify(number: usize) -> Option<Solution> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(number * BASE).to_string());
    let hash: String = hasher.result_str();
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else { None }
}

fn find(
    start_at: usize,
    sender: mpsc::Sender<Solution>,
    is_solution_found: Arc<AtomicBool>,
) {
    for number in (start_at..).step(THREADS) {
        if is_solution_found.load(Ordering::Relaxed) { return; }
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
    let is_solution_found = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = mpsc::channel();
    println!("Started {} threads", THREADS);
    for i in 0..THREADS {
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            find(i, sender_n, is_solution_found);
        });
    }
    println!("Please wait...  ");
    match receiver.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution: ");
            println!("\tThe number is: {}", i);
            println!("\tthe hash result is : {}.", hash);
        }
        Err(_) => panic!("Worker threads disconnected!"),
    }
}
