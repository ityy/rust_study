#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            qdata: Vec::new()
        }
    }

    fn push(&mut self, item: T) {
//        let a=self.qdata;//cannot move
//        let mut a = &self.qdata;//borrow is ok
//        a.push(T);
        self.qdata.push(item); //直接链式调用,会根据后边所需,前边自动返回所需类型. push需要&mut, 则self.qdata返回的即为&mut.
    }

    fn pop(&mut self) -> T {
        self.qdata.remove(0)
    }
}

pub fn main() {
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);
    q.pop();
}