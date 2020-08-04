//! # 内存泄漏
//! 基于链表，演示内存泄漏
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

/// # 节点类型别名
/// Option表示节点可空, Rc表示节点可以引用计数, 由于Rc不能修改, 由RefCell提供内部可变性
type NextNode<T> = Option<Rc<RefCell<Node<T>>>>;

/// # 链表节点
struct Node<T: Display> {
    data: T,
    next: NextNode<T>,
}

/// # 实现方法
impl<T: Display> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            next: None,
        }
    }
}

/// # 实现drop特性
/// 当drop时打印一条提示信息
impl<T: Display> Drop for Node<T> {
    fn drop(&mut self) {
        println!("dropping node, data:{}", &self.data);
    }
}

/// # 封装一个转换方法
fn convert_to_next_node<T: Display>(node: Node<T>) -> NextNode<T> {
    let next = Rc::new(RefCell::new(node));
    Some(next)
}


/// # 测试drop打印信息
/// 观察得到释放顺序:
/// 1. second
/// 2. first
#[test]
fn drop_info_test() {
    let first = Node::new(1);
    let second = Node::new(2);
}

/// # 演示内存泄露（memory leak）
/// 释放顺序:
/// 1. second
/// 2. first
///
/// 泄露分析：
/// 1. second释放 由于first有second的一个clone, second只是计数-1
/// 2. first释放 由于second有first的一个clone, first只是计数-1
/// 3. 因引用计数均不为0，drop没有被执行。second和first没有被内存回收。
/// 4. 函数正常结束。程序最终失去对这部分内存的主动管理，导致内存泄漏。
#[test]
fn test() {
    let first = Rc::new(RefCell::new(Node::new(1)));
    let second = Rc::new(RefCell::new(Node::new(2)));

    //下面的代码导致互相引用, drop最终没有被执行。注释任意一句破除互相引用, 就不会内存泄漏
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());
}
