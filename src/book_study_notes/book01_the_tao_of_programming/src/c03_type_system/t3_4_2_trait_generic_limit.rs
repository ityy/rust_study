//! # trait 系统：泛型限定
//! 使用trait对泛型进行限定，叫做trait Bound (特性限定)。
use std::ops::Add;

/// ## 泛型限定
/// 如下函数，T是整数等可加的类型时没有问题，T是其它无法相加的类型时怎么办？我们可以对泛型T进行限定。
/// ```
/// fn sum<T>(a:T,b:T)->T{
///     a+b
/// }
/// ```
/// 我们使用<T: Add<T, Output=T>>对T进行了限制， 只有实现了Add特性的类型，且当中的Rhs（右值类型）和Output（返回值类型）都为T时，才能使用此sum函数。<br/>
/// 当需要限定多个特性时，特性之间可以使用 + 号连接。
fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b
}
