use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

//全局静态变量宏, 支持vec, hashmap
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




///智能指针的地址
#[test]
fn test_number_arc() {
    let num: i32 = 5; //num这个符号就是一个内存地址,我们不能拿到这个地址
    println!("num:{}", num);//打印num这个内存地址上的值

    {
        //&num生成一个指针, as *const i32 将其转为原生指针并赋予指针类型, as usize 将指针转为整数数值
        let num_addr_int = &num as *const i32 as usize;
        println!("num_addr：0x{:X}", num_addr_int);//以Hex形式打印
    }
    {
        //&num生成一个指针, as *const i32 将其转为原生指针并赋予指针类型, as usize 将指针转为整数数值
        let num_addr_int = &num as *const i32 as usize;
        println!("num_addr：0x{:X}", num_addr_int);//以Hex形式打印
    }
    //&num生成一个指针, as *const i32 将其转为原生指针并赋予指针类型, as usize 将指针转为整数数值
    let num_addr_int = &num as *const i32 as usize;
    println!("num_addr：0x{:X}", num_addr_int);//以Hex形式打印

    ///Arc改变了num的地址
//    let num_arc = Arc::new(num);
//    let num_arc_addr_int = num_arc.deref() as *const i32 as usize;
//    println!("num_arc_addr_int：0x{:X}", num_arc_addr_int);//以Hex形式打印
//    let num_arc_clone_addr_int = num_arc.clone().deref() as *const i32 as usize;
//    println!("num_arc_clone_addr_int：0x{:X}", num_arc_clone_addr_int);//以Hex形式打印


    ///Box改变了num的地址
    let num_box = Box::new(num);
    let num_box_addr_int = num_box.deref() as *const i32 as usize;
    println!("num_box_addr_int：0x{:X}", num_box_addr_int);//以Hex形式打印


    //为了验证刚才的地址是不是正确的，我们修改这个指针指向的数据
    //num_ptr就是addr对应的raw指针
    let num_ptr = num_addr_int as *mut i32;
    unsafe {
//        let num_mut = &mut *num_ptr;
//        *num_mut = 100

        *num_ptr = 100;//解引用，给他赋值100
    };

    println!("num:{}", num);//num已经变成100了
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
