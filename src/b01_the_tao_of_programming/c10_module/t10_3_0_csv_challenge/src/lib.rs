//! 使用lib.rs文件的方式组织各模块，再在main.rs中使用lib即可
//! 或者直接在main.rs中组织各模块，两种方式等价。
//! This is documentation for the `csv_challenge` lib crate.
//!
//! Rust集成测试
//! 二进制包（即main）是不能增加集成测试的，只有库（即lib）才可以
//! 这里将main.rs改造为main.rs+lib.rs的模式，这也是二进制包的最佳实践。
//! 增加集成测试:
//!     在根目录下创建tests文件夹。即与src文件夹同级。
//!     使用cargo test 即可执行集成测试。(此命令会执行代码中的所有test，以及tests文件夹中的所有test，以及文档注释中的所有代码，除非使用ignore进行忽略)
//!
//! Usage:
//! ```ignore
//!     use csv_challenge::{
//!         Opt,
//!         {load_csv, write_csv},
//!         replace_column,
//!     };
//! ```
pub use self::core::{
    read::{load_csv, write_csv},
    write::replace_column,
};
// Re-exporting
pub use self::opt::Opt;

mod opt;
mod err;
mod core;
