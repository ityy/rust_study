use std::sync::Mutex;

#[test]
pub fn test() {
    let m = Mutex::new(5);
    {
        //锁仅在同级作用域保持
        let mut num = m.lock().unwrap();
        *num = 6;
    }//锁释放
    println!("m={:?}", m);
}