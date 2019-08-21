use std::collections::HashMap;

pub fn main() {
    stack_test()
}


fn stack_test() {
    //申请一个内存地址x,存放数字5
    let mut x = 5;
    //&操作符 就是取x的地址
    //申请一个内存地址y,存放x的地址
    let y = &x;
    //y2和z里面的地址不一定一样, &&x的时候生成了另一条指针链路.
    let y2 = &y;

    let addr = &x as *const i32 as usize;
    println!("&x addr：0x{:X}", addr);
    let addr = y as *const i32 as usize;
    println!("y addr：0x{:X}", addr);
    let addr = &y as *const &i32 as usize;
    println!("&y addr：0x{:X}", addr);
    let addr = y2 as *const &i32 as usize;
    println!("y2 addr：0x{:X}", addr);
    let addr = &&y as *const &&i32 as usize;
    println!("&&y addr：0x{:X}", addr);

    //多个&操作符, 可以生成中间串联的多个地址, 即使没有存放到某个变量.
    //申请一个内存地址z, 将指针连的最后一个地址存放到z里
    let z = &&&x;
    //另一条指针链路
    let addr = z as *const &&i32 as usize;
    println!("z addr：0x{:X}", addr);
    let addr = *z as *const &i32 as usize;
    println!("*z addr：0x{:X}", addr);
    let addr = **z as *const i32 as usize;
    println!("**z addr：0x{:X}", addr);

    // 打印结果:
    // &x addr：0xF000EFF8A4
    // y addr：0xF000EFF8A4
    // &y addr：0xF000EFF8A8
    // y2 addr：0xF000EFF8A8
    // &&y addr：0xF000EFFA20
    // z addr：0xF000EFFA80
    // *z addr：0xF000EFFA88
    // **z addr：0xF000EFF8A4
}


fn heap_test() {
    let x: HashMap<i32, i32> = HashMap::new();
    //与stack测试没有区别
    //变量符号代表一个内存地址
}