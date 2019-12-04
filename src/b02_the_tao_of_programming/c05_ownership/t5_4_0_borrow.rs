//! 所有权借用
//! 引用&x也可称为x的借用。所有权借用有如下限制：
//!     在不可变借用期间，所有者不可修改资源，也不可进行可变借用。
//!     在可变借用期间，所有者不能访问资源，也不能再出借所有权。
//! 引用在离开作用域之时，就是归还所有权之时。
//!
//! 关于借用，为了保证内存安全有三个规则
//!     1 借用的生命周期不能长于出借方的生命周期 (防止悬垂指针)
//!     2 可变借用不能有别名（alias），因为可变借用有独占性 （可变不共享）
//!     3 不可变借用不能再次出借为可变借用 （共享不可变）

///借用检查 保障了内存安全
#[test]
fn test_borrow_check() {
    fn compute(input: &u32, output: &mut u32) {
        if *input > 10 {
            *output = 1;
        }
        if *input > 5 {
            *output *= 2;
        }
    }

    let x = 20;
    let mut y = 5;
    compute(&x, &mut y);
    println!("{}", y);//2
    //假如取消借用检查，下面的代码会打印1而不是2。
    //compute(&y, &mut y);// error[E0502]: cannot borrow `y` as mutable because it is also borrowed as immutable
    //println!("{}", y);//1
}


/// 解引用操作会获得所有权
#[test]
fn test_deref() {
    fn join(s: &String) -> String {
        //解引用操作会获得所有权，这里s是不可变借用，编译器不允许转移所有权，否则外部所有者就被篡权了。
        // ```
        //let append = *s;//error[E0507]: cannot move out of `*s` which is behind a shared reference
        //"hello".to_string() + &append
        // ```
        // 编译器不允许move语义的发生，但不影响使用：
        "hello ".to_string() + &*s
    }

    let x = "world".to_string();
    let result = join(&x);
    println!("{}", result);
}

