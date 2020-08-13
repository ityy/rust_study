//! # 迭代器
//! 在Rust语言中，闭包最常用的场景是在遍历集合容器中的元素时，按闭包给定的逻辑进行操作。<br/>
//! 传统for循环中，需要一个变量（例如i）来记录每一次迭代所在的位置。<br/>
//! 许多新型编程语言，已经通过模式化的方式来返回迭代过程中的每一个元素。这种模式化，称为迭代器（iterator）模式。也被称为游标（Cursor）模式。
//!
//! # 迭代器分为外部迭代器和内部迭代器。
//! 外部迭代器:
//! > 独立于容器之外，利用容器提供的方法（比如next方法就是所谓的游标）来迭代下一个元素，可以控制整个遍历的过程。java即为此类。
//! 内部迭代器:
//! > 通过迭代器自身来控制迭代下一个元素，只要调用了内部迭代器，并通过闭包传入相关操作，就必须等待迭代完成才可以停止遍历。
//!
//! Rust早期使用内部迭代器，加上Rust的所有权系统，导致使用起来很复杂，后来改为外部迭代器，也就是for循环。


/// # 内部迭代器演示
#[test]
fn test_internal_iterator() {
    //定义一个特性
    trait InternalIterator<T: Copy> {
        //定义一个方法，参数为一个闭包
        fn each<F: Fn(T) -> T>(&mut self, f: F);
    }

    //为Vec实现此特性
    impl<T: Copy> InternalIterator<T> for Vec<T> {
        fn each<F: Fn(T) -> T>(&mut self, f: F) {
            let mut i = 0;
            while i < self.len() {
                self[i] = f(self[i]);
                i += 1;
            }
        }
    }

    //测试
    let mut v = vec![1, 2, 3];
    v.each(|i| i * 3);
    println!("{:?}", v);

    // 满足特性孤儿规则：特性InternalIterator在当前crate内定义。
}

/// # 外部迭代器演示
/// Rust中的for循环本质是一个语法糖，其代码展开形式如下：
#[test]
fn test_expand_for() {
    //容器
    let v = vec![1, 2, 3, 4, 5];

    //for 外部迭代 遍历容器
    for i in &v {
        println!("{}", i);
    }

    //for展开的代码
    {
        let mut _iterator = v.into_iter();
        loop {
            match _iterator.next() {
                Some(i) => println!("{}", i),
                None => break,
            }
        }
    }
}

/// # Iterator trait
/// 基于for语法糖的特性，及展开后的代码，会发现Rust是通过trait来实现的迭代器模式。
///
/// # 为自定义类型实现迭代器
#[test]
fn impl_iterator() {
    struct Counter {
        count: usize,
    }

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 { Some(self.count) } else { None }
        }
    }

    let counter = Counter { count: 1 };
    for i in counter.into_iter() {
        println!("{}", i);
    }
}


/// IntoIterator trait
/// 如果想迭代某个集合容器中的元素，必须将其转换为迭代器才可以使用。那么迭代器到底是什么？
/// 前面讲过From和Into互为反操作，但迭代器没有用到这两个特性，而是使用了两个新特性：
///     FromIterator
///     IntoIterator
///
#[test]
fn test_into_iterator() {
    // 点进源码发现实现了Iterator特性的类型会自动实现IntoIterator特性
}



