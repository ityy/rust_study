//! # 读操作模块

/// 使用父模块(mod.rs)空间下的目标
use super::{Error, File, PathBuf, Read};

/// # 从文件读取csv内容
/// Usage:
/// ```ignore
/// use std::path::PathBuf;
/// let filename = PathBuf::from("./files/challenge.csv");
/// let csv_data = load_csv(filename);
/// assert!(csv_data.is_ok());
/// ```
pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let file = read(csv_file)?;
    Ok(file)
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        return Err("input file missing")?;
    }
    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}


/// # 单元测试示例
/// '#[cfg(xxx)]' 仅在执行cargo xxx命令时才会编译执行的代码
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    /// 使用父模块(read.rs)空间下的目标
    use super::load_csv;

    /// # 测试文件读取是否正常
    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }

    #[test]
    // #[ignore]  //忽略标记 可以忽略此测试函数
    fn test_invalid_load_csv() {
        let filename = PathBuf::from("./input/no_file.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_err());
    }
}