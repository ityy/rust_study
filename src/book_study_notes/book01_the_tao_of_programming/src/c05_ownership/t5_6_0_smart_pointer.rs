//! # 智能指针与所有权
//! 除了普通引用（借用），Rust还提供了智能指针（移动）。

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// # Box 唯一解引用会移动所有权的指针
#[test]
fn test_box() {
    let x = "hello".to_string();

    // 智能指针独占所有权
    let y = Box::new(x);
    //println!("{}", x); //error[E0382]: borrow of moved value: `x`

    //Box解引用会转移所有权
    let z = *y;
    //println!("{}", y); //error[E0382]: borrow of moved value: `y`
    println!("{}", z);
}

/// # Rc、Arc 共享所有权
/// 引用计数是简单的GC算法之一。<br/>
/// Rc<T>可以共享所有权，只有当引用计数为0时，目标才会被析构。
#[test]
fn test_rc() {
    ///移动目标到函数，且不返回，使其销毁
    fn drop_rc(t: Rc<i32>) {
        t;
    }

    let x = Rc::new(45);
    let y1 = x.clone();//增加强引用
    let y2 = x.clone();//增加强引用
    println!("x:{} y1:{} y2:{}", x, y1, y2);

    // 强引用数量
    println!("{:?}", Rc::strong_count(&x)); //3 包含x y1 y2
    // 销毁x
    drop_rc(x);
    // 强引用数量
    println!("{:?}", Rc::strong_count(&y1)); //2 包含y1 y2 因为x被销毁

    // 增加弱引用，弱引用没有所有权
    let _z1 = Rc::downgrade(&y1);
    // 弱引用数量
    println!("{:?}", Rc::weak_count(&y1)); //1 包含z1

    let y3 = &*y1; //不增加计数
    println!("{} {}", y1, y3);
    println!("{:?}", Rc::strong_count(&y1)); //2 包含y1 y2
}

/// # 内部可变性
/// 当结构体本身不可变，但成员需要可变时怎么办？Rust提供了内部可变性。
/// ## 值语义：Cell<T> 引用语义：RefCell<T>
/// 其实它们本质上不属于智能指针，是一种容器，是Rust中的一种设计模式。
#[test]
fn test_cell() {
    #[derive(Debug)]
    /// # 值语义使用Cell
    /// 我们通过get/set方法对其操作，非常类似OOP语言中的getter/setter方法。无运行时消耗。
    struct Foo {
        x: u32,
        y: Cell<u32>,
    }
    let foo = Foo { x: 1, y: Cell::new(3) };
    println!("{:?}", foo);
    foo.y.set(42);
    println!("{:?}", foo);
}

#[test]
fn test_refcell() {
    #[derive(Debug)]
    /// # 引用语义使用RefCell
    /// 通过borrow_mut或borrow来获取引用。有运行时借用检查消耗，检查不通过会引起线程panic
    struct Foo {
        x: u32,
        y: RefCell<String>,
    }
    let foo = Foo { x: 1, y: RefCell::new("hello".to_string()) };
    println!("{:?}", foo);
    let mut s = foo.y.borrow_mut();
    s.push_str("world");
    println!("{:?}", foo); //此处y显示被借走
    drop(s);//销毁借用
    println!("{:?}", foo); //此处y显示正常
    /*结果：
        Foo { x: 1, y: RefCell { value: "hello" } }
        Foo { x: 1, y: RefCell { value: <borrowed> } }
        Foo { x: 1, y: RefCell { value: "helloworld" } }
     */
}


/// # Cow<T> 写时复制
/// 写时复制（copy on write）技术是一种程序优化的策略。<br/>
/// Cow<T>是一个枚举体的智能指针，有两个可选值：
/// - Borrowed    用于包裹引用
/// - Owned       用于包裹所有者
///
/// Cow<T>提供的功能是：以不可变的方式访问借用内容，在需要可变或所有权的情况下克隆一份数据。适合读多写少的场景。<br/>
/// 注意：与java的写时复制容器不同，java的写带锁，且写完之后引用会指到新的内容上，主要为了解决写的时候不影响读旧数据的问题，自写完的那刻起，所有的读都是读的新内容了。
#[test]
fn test_cow() {
    /// 将数组的值全部取绝对值
    fn abs_all(input: &mut Cow<[i32]>) {
        for i in 0..input.len() {
            let v = input[i];
            if v < 0 {
                // 当需要修改时，使用to_mut()方法会克隆一份数据，以拥有新数据的所有权。
                // 如果本身就拥有所有权，或已经克隆过新数据掌握了所有权，不会再次克隆。
                // 也可以用into_owned()方法获取一个所有权对象。如果T是借用的，则会克隆一份。如果T拥有所有权，则会移动。
                input.to_mut()[i] = -v;
            }
        }
    }

    // 都是正数，没有可变需求, 不会发生克隆
    let s1 = [1, 2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1); //s1 i1仍是同一对象

    // 有负数，有可变需求，且i2没有s2的所有权，会发生克隆
    // 注意:借用数据被克隆为了新的对象
    let s2 = [1, 2, 3, -45, 6];
    let mut i2 = Cow::from(&s2[..]);//Cow内只是s2的引用，没有所有权。所以在发生修改时，Cow会自动克隆一份数据。
    abs_all(&mut i2);
    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2); //s2 i2已不是同一对象

    /// 求绝对值之和
    fn abs_sum(ns: &[i32]) -> i32 {
        let mut lst = Cow::from(ns);
        abs_all(&mut lst);
        lst.iter().fold(0, |acc, &n| acc + n)
    }
}