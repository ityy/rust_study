//! Rust集成测试
//! 二进制包（即main）是不能增加集成测试的，只有库（即lib）才可以
//! 这里将main.rs改造为main.rs+lib.rs的模式，这也是二进制包的最佳实践。
//! 增加集成测试:
//!     在根目录下创建tests文件夹。即与src文件夹同级。
//!     使用cargo test 即可执行集成测试。(此命令会执行代码中的所有test，以及tests文件夹中的所有test，以及文档注释中的所有代码，除非使用ignore进行忽略)

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use t10_3_0_csv_challenge::{
        {load_csv, write_csv},
        Opt,
        replace_column,
    };

    #[test]
    fn test_load_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }

    #[test]
    fn test_replace_column() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing");
        assert!(modified_data.is_ok());
    }

    #[test]
    fn test_write_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
        let output_file = write_csv(&modified_data, "output/test.csv");
        assert!(output_file.is_ok());
    }
}
