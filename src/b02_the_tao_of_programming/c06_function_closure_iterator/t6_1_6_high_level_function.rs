//! 高阶函数
//! 在数学中，高阶函数也叫算子或泛函。比如微积分中的导数就是一个函数到另一个函数的映射。
//! 在计算机科学里，高阶函数是指以函数作为参数或返回值的函数，它也是函数式编程语言最基础的特性。

/// 函数指针
/// 在Rust中，函数是一等公民。
/// 这表示函数也可以作为函数的参数和返回值使用。
#[test]
pub fn function_is_variable() {
    //定义函数
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    };
    fn mul(a: i32, b: i32) -> i32 {
        a * b
    };

    //函数指针与函数类型
    let sum_fn_pointer: fn(i32, i32) -> i32 = sum;//显式指定类型，则sum_fn为函数指针
    println!("{:p}", sum_fn_pointer);//可以打印指针地址
    let sum_fn = sum;//不指定类型，则sum_fn为函数类型
    //println!("{:p}", sum_fn);//error 不能打印指针地址
    //在调用时没有区别
    sum_fn_pointer(1, 2);
    sum_fn(1, 2);

    ///函数作为参数
    fn math(operation: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        //operation指定了类型，则operation为函数指针
        operation(a, b)
    }
    let result_sum = math(sum_fn_pointer, 1, 2);
    let result_mul = math(mul, 1, 2);
    println!("{}", result_sum);
    println!("{}", result_mul);

    //使用type给函数指针定义别名
    type MathOp = fn(i32, i32) -> i32;
    fn math2(op: MathOp, a: i32, b: i32) -> i32 {
        println!("{:p}", op);
        op(a, b)
    }
    let result_sum = math2(sum, 1, 2);
    println!("{}", result_sum);
}

///函数作为返回值
#[test]
fn fuction_is_return_value() {
    fn is_true() -> bool {
        true
    }
    //函数作为返回值的函数
    fn true_maker() -> fn() -> bool {
        //返回一个函数
        is_true
    }
    println!("{}", true_maker()());//这里没问题，调用函数true_maker返回了一个函数，继续调用
}


