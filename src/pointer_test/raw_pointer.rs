use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    //带互斥锁的map,可以获取锁后进行读写操作
    static ref LIB_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    //这个库声明的都是静态不可变引用, 下方的map只能读不能写, 可以在初始化时添加数据
    //但可以使用c指针(即原始指针)的形式破除不可写的限制
    static ref LIB_MAP_S: HashMap<String, String> =HashMap::new();
    //创建一个独立锁. 由于锁是在作用域结束后自动解锁, 可以单独作为锁使用.
    static ref LIB_MAP_LOCK: Mutex<i32> = Mutex::new(0);

    //下方代码可以正常编译 运行时报错误退出 原因是这个地址存放的map被释放了
//    static ref LIB_MAP_S: usize ={
//        let mut map:HashMap<String, String>=HashMap::new(); //这里声明的map 在下方花括号处被释放了...
//        map.insert("c".to_string(), "ccc".to_string());
//        &mut map as *mut  HashMap<String, String> as usize
//    }; //由于返回的是一个裸指针,map在这里被drop了,导致无法使用
}



pub fn test() {
    test_Hashmap();
}


///使用原生指针
/// 打印变量的内存地址
fn test_number() {
    let a: i32 = 5;
    //&a先转成raw指针，然后再把指针转成usize，这个可以print的
    let addr = &a as *const i32 as usize;
    println!("addr：0x{:X}", addr);

    //为了验证刚才的地址是不是正确的，我们修改这个指针指向的数据
    //pa就是addr对应的raw指针
    let pa = addr as *mut i32;
    //解引用，给他赋值100
    unsafe {
        //由于i32是在栈上,直接用*pa=100页可以
        let num = &mut *pa;
        *num = 100
    };

    //打印a，可以看到a已经变成100了
    println!("value:{}", a);
}

fn test_Hashmap() {
    let a: HashMap<String, String> = HashMap::new();

    //&a先转成raw指针，然后再把指针转成usize，这个可以print的
    let addr = &a as *const HashMap<String, String> as usize;
    println!("addr：0x{:X}", addr);

    //为了验证刚才的地址是不是正确的，我们修改这个指针指向的数据
    //pa就是addr对应的raw指针
    let pa = addr as *mut HashMap<String, String>;
    unsafe {
        //解裸指针, 转换为&mut引用
        let map = &mut *pa;
        map.insert("a".to_string(), "aaa".to_string())
    };

    //打印a，可以看到a已经变成100了
    println!("value:{:#?}", a);
}


//测试读全局map
fn map_read() {
    let LIB_MAP_P: usize = &*LIB_MAP_S as *const HashMap<String, String> as usize;
    let pointer: *mut HashMap<String, String> = LIB_MAP_P as *mut HashMap<String, String>;
    unsafe {
        let map = &mut *pointer;
        let k = map.get("a").unwrap();
        let s = map.get("b").unwrap();
        println!("{} {}", k, s);
    }
}

//测试写全局map
fn map_write() {
    let LIB_MAP_P: usize = &*LIB_MAP_S as *const HashMap<String, String> as usize;
    println!("map_addr:0x{:X}", LIB_MAP_P);//打印map的内存地址
    let pointer: *mut HashMap<String, String> = LIB_MAP_P as *mut HashMap<String, String>;
    unsafe {
        let map = &mut *pointer;
        map.insert("a".to_string(), "aaa".to_string());
        map.insert("b".to_string(), "bbb".to_string());
    }
}

