//! 内存泄漏测试
//! 基于链表
use std::cell::RefCell;
use std::rc::Rc;

//类型别名
//Option表示节点可空, Rc表示节点可以引用计数, 由于Rc不能修改, 由RefCell保证内部可变性
type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: NodePtr<T>,
}

//实现drop, 当drop时打印一条提示信息
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("dropping!");
    }
}


#[test]
fn test() {
    let first = Rc::new(RefCell::new(Node {
        data: 1,
        next: None,
    }));
    let second = Rc::new(RefCell::new(Node {
        data: 2,
        next: None,
    }));

    //没有下面的互相引用, drop正常执行.
    //下面的代码导致互相引用, drop最终没有被执行, 导致内存泄漏.
    first.borrow_mut().next = Some(second.clone());//注释任意一句破除互相引用, 就不会内存泄漏
    second.borrow_mut().next = Some(first.clone());
}
//释放顺序 second first
//1 second释放 由于first有second的一个clone, second只是计数-1
//2 first释放 由于second有first的一个clone, first只是计数-1
//3 程序结束


