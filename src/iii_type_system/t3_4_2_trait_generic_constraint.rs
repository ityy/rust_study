//! 泛型约束
//! 使用trait对泛型进行约束，叫做trait Bound (特性限定)
use std::ops::Add;

///泛型约束
///
/// ```
/// fn sum<T>(a:T,b:T)->T{
///     a+b
/// }
/// ```
/// 上述函数，T是整数等可加的类型时没有问题，T是其它无法相加的类型时怎么办？
/// 所以要对T加以限制。
/// 用trait作为泛型的约束
fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b
}
///我们使用<T: Add<T, Output=T>>对T进行了限制， 只有实现了Add特性的类型，且当中的Rhs和Output都为T时，才能使用此sum函数
///特性限定中，+号表示且的意思
