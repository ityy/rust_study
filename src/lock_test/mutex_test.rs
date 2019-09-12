use std::sync::Mutex;

#[test]
pub fn test() {
    let m = Mutex::new(5);
    {
        //锁仅在同级作用域保持
        let mut num = m.lock().unwrap();
        *num = 6;
    }//锁释放

    //再次获取锁没问题
    let mut num2 = m.lock().unwrap();

    println!("m={:?}", m);
}


#[test]
pub fn test2() {
    let m = Mutex::new(5);
    let mut num = m.lock().unwrap();
    *num = 6;
    //再次获取锁阻塞 直接死锁 所以单线程异步不能用多线程的锁, 会把单线程阻塞死
    let mut num2 = m.lock().unwrap();
    println!("m={:?}", m);
}

