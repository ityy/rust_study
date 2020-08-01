//! # 所有权借用
//! 引用语义类型有一套所有权借用规则。引用&x也可称为x的借用。所有权借用有如下限制：
//! - 在不可变借用期间，所有者不可修改资源，也不可进行可变借用。（不可变借用共享性）
//! - 在可变借用期间，所有者不能访问资源，也不能再出借所有权。（可变借用排它性，在编译期由编译器检查）
//!
//! 引用在离开作用域之时，就是归还所有权之时。<br/>
//!
//! 关于借用，为了保证内存安全有三个规则：
//! - 1 借用的生命周期不能长于所有者的生命周期 (防止悬垂指针)
//! - 2 可变借用不能有别名（alias），因为可变借用有独占性 （可变借用排它性）
//! - 3 不可变借用不能再次出借为可变借用 （不可变借用共享性）

/// # 借用检查 保障了内存安全
#[test]
fn test_borrow_check() {
    /// ## 值语义类型的引用：
    /// input的值为x的地址，*input为取x的值。
    ///
    /// 地址：|  x |  <br/>
    /// 值： | 20 |
    ///
    /// 地址：|  input |  <br/>
    /// 值： |    &x  |
    ///
    /// *input=*&x=x=20
    fn compute(input: &u32, output: &mut u32) {
        if *input > 10 {
            *output = 1;
        }
        if *input > 5 {
            *output *= 2;
        }
    }

    let x = 20;
    let mut y = 20;

    println!("&x is {:p}", &x);
    // 有借用检擦时，确保x、y都按规则使用
    compute(&x, &mut y);
    println!("{}", y);//2

    // 若无借用检查，下面的代码会打印1而不是2
    //compute(&y, &mut y);// error[E0502]: cannot borrow `y` as mutable because it is also borrowed as immutable
    //println!("{}", y);//1
}


/// # 解引用操作会获得所有权
#[test]
fn test_deref() {
    /// ## 引用语义类型的引用：
    /// input的值为x的地址，*input为取x的值。x的值为String胖指针，指向堆内存上的字符串。
    ///
    /// 地址：|  String胖指针  |  <br/>
    /// 值： |    "world"    |
    ///
    /// 地址：|      x      |  <br/>
    /// 值： | String胖指针 |
    ///
    /// 地址：|  input |  <br/>
    /// 值： |    &x  |
    ///
    /// *input=*&x=x
    ///
    /// 解引用操作会获得所有权，这里s是不可变借用，编译器不允许发生move语义转移所有权，否则外部所有者就被篡权了。
    ///  ```
    ///  fn join(input: &String) -> String {
    ///     let append = *input;//error[E0507]: cannot move out of `*input` which is behind a shared reference
    ///     "hello".to_string() + &append
    ///  }
    ///  ```
    /// 这里没有发生move，没有新变量夺取所有权，编译器检查通过。
    fn join(input: &String) -> String {
        "hello ".to_string() + &**input
    }

    let x = "world".to_string();
    let result = join(&x);
    println!("{}", result);
}

