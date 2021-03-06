//! # Vec 动态数组
//! Rust中的数组有两种类型:
//! ```ignore
//! array       长度固定    签名为[T;N]    可以在栈上存储
//! vec         动态长度    签名为Vec<T>   运行时确定大小，只能在堆上存储
//! ```
//! Vec即vector，意为矢量。矢量为具有大小和方向的量。<br/>
//! 创建Vec，与创建String类似，因为String本质上是对Vec<u8>的包装。




/// # Vec 基本操作
/// ```
///   //新建
///  let v1: Vec<i32> = Vec::new(); //new 方式
///  let mut v2 = vec![1, 2, 3]; //宏 方式
///
///  //增加
///  v2.push(4);
///
///  //获取
///  let i1 = &v2[1]; // 下标获取
///  let i2 = v2.get(2).unwrap(); // get方法获取, 返回option
///
///  //遍历
///  for i in &v2 {
///      println!("{}", i);
///  }
/// ```
#[test]
fn test() {
    // push、get
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec[0], 1);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    vec[0] = 7;
    assert_eq!(vec.get(0), Some(&7));
    assert_eq!(vec.get(0..1), Some(&[7][..]));// [..]是对&[7]的切片，以达到左右两边类型相同的目的。
    assert_eq!(vec.get(0..2), Some(&[7, 2][..]));
    assert_eq!(vec.get(10), None);
    //vec[10]; // error

    // 扩展vec
    vec.extend([1, 2, 3].iter().clone());
    assert_eq!(vec, [7, 1, 2, 3]);
    assert_eq!(vec.get(0..2), Some(&[7, 1][..]));

    // 追加vec
    let mut vec2: Vec<i32> = vec![4, 5, 6];
    vec.append(&mut vec2); // 将vec2的元素移动到vec
    assert_eq!(vec, [7, 1, 2, 3, 4, 5, 6]);
    assert_eq!(0, vec2.len());
}

/// # 容量
/// Vec::with_capacity(N) 预分配堆内存空间 <br/>
/// 用于避免频繁申请内存空间会浪费性能的场景 <br/>
/// 所谓集合容器，一定是有容量的概念才称作容器。
/// -   容量（capacity）        指已分配的内存空间大小
/// -   大小/长度（size/length） 指已包含的元素个数
///
/// 容器的扩容和收缩一般不需要开发者关心，但也是编程时需要考虑的性能问题。
#[test]
fn test_with_capacity() {
    let mut vec: Vec<i32> = Vec::with_capacity(10);
    assert_eq!(0, vec.len());//长度
    assert_eq!(10, vec.capacity());//容量
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(10, vec.len());
    assert_eq!(10, vec.capacity());

    vec.truncate(0); //从给定索引值开始截断 不会释放预分配的内存
    assert_eq!(0, vec.len());
    assert_eq!(10, vec.capacity());
    vec.push(0);
    assert_eq!(1, vec.len());
    assert_eq!(10, vec.capacity());

    vec.clear(); //清空容器 不会释放预分配的内存
    assert_eq!(0, vec.len());
    assert_eq!(10, vec.capacity());

    vec.shrink_to_fit(); //收缩到合适容量 会释放没有元素的内存。相当于重新分配内存。
    assert_eq!(0, vec.len());
    assert_eq!(0, vec.capacity());
}

