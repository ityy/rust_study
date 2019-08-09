///多模块测试
pub mod test1 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in ((i + 1)..nums.len()) {
                if &nums[i] + &nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }
}

///将函数赋值给变量
/// 将函数作为参数传递
pub fn main() {
    //将x与函数绑定
    let x = func_type;
    //调用函数将x传递进去
    exec_func(x);
}

//声明一个函数
fn func_type(x: i32) -> i32 {
    x + 1
}

//声明一个函数 参数也是一个函数,规定了入参和返回值
fn exec_func(f: fn(i32) -> i32) {
    println!("exec a func type: {}", f(5));
}
