//! 内部可变性探究
//! UnsafeCell<T> 源码解读
//! Mutex,Rwlock,Cell,RefCell,原子类型以及mpsc::Sender等的源码内部，都是使用UnsafeCell来提供内部可变性。
//! UnsafeCell是将不可变转为可变的唯一合法渠道，上述类型不会违反Rust的安全检查。

/// //语言项，编译器中有针对unsafe_cell的特殊处理，可以认为是照顾，此时关闭相关的检查。
/// #[lang = "unsafe_cell"]
/// #[stable(feature = "rust1", since = "1.0.0")]
/// #[repr(transparent)]
/// pub struct UnsafeCell<T: ?Sized> {
///     value: T,
/// }
///
/// //UnsafeCell本身被标记为!Sync，单独使用UnsafeCell不能通过线程安全检查。
/// #[stable(feature = "rust1", since = "1.0.0")]
/// impl<T: ?Sized> ! Sync for UnsafeCell<T> {}
///
/// impl<T> UnsafeCell<T> {
///     #[stable(feature = "rust1", since = "1.0.0")]
///     #[inline]
///     pub const fn new(value: T) -> UnsafeCell<T> {
///         UnsafeCell { value }
///     }
///
///     #[inline]
///     #[stable(feature = "rust1", since = "1.0.0")]
///     pub fn into_inner(self) -> T {
///         self.value
///     }
/// }
///
/// impl<T: ?Sized> UnsafeCell<T> {
///     // 最重要的方法 get，将不可变转为可变。
///     // UnsafeCell是将不可变转为可变的唯一合法渠道。
///     #[inline]
///     #[stable(feature = "rust1", since = "1.0.0")]
///     pub const fn get(&self) -> *mut T {
///         self as *const UnsafeCell<T> as *const T as *mut T
///     }
/// }
///
fn nothing() {}