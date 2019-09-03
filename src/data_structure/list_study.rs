//省略枚举路径
use List::*;

enum List {
    //包含一个元素和下一个节点的指针
    Cons(u32, Box<List>),
    //末端标识
    Nil,
}

impl List {
    fn new() -> List { Nil }

    ///前插
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // `self` 的类型是 `&List`, `*self` 的类型是 `List`,
        // 匹配一个类型 `T` 好过匹配一个引用 `&T`
        match *self {
            // 因为`self`是借用的，所以不能转移 tail 的所有权
            // 因此使用 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),//采用递归遍历整个链
            // 基本规则：所以空的链表长度都是0
            Nil => 0
        }
    }

    // 返回连链表的字符串表达形式
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 很像
                // 但是返回一个堆上的字符串去替代打印到控制台
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

#[test]
fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}