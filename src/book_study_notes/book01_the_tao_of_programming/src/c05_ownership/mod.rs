//! # 第五章 所有权系统
//! C++ 的 RAII 机制解决了无 GC 的内存自动管理的基本问题，但没有解决全部问题。<br/>
//! Rust 引入所有权系统，解决了悬垂指针等问题，保障了内存安全。


mod t5_1_0_ownership;
mod t5_3_0_let_scope_lifetime;
mod t5_4_0_borrow;
mod t5_5_0_lifetime_parameter;
mod t5_6_0_smart_pointer;