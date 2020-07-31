/// 使用父模块已定义的类型
use super::{Error, File, PathBuf, Read, Write};

/// # Usage:
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

/// # Usage:
/// ```ignore
/// let filename = PathBuf::from("./files/challenge.csv");
/// let csv_data = load_csv(filename).unwrap();
/// let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
/// let output_file = write_csv(&modified_data, "output/test.csv");
/// assert!(output_file.is_ok());
/// ```
pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error>
{
    write(csv_data, filename)?;
    Ok(())
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

fn write(data: &str, filename: &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}

/// 单元测试示例
/// 测试模块
/// #[cfg(xxx)] 仅在执行cargo xxx命令时才会编译执行的代码
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{load_csv, write_csv};

    /// 测试函数标记
    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }

    /// 测试函数标记
    #[test]
//    #[ignore]  //忽略标记 可以忽略此测试函数
    fn test_invalid_load_csv() {
        let filename = PathBuf::from("./input/other.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_err());
    }

    /// 测试函数标记
    #[test]
    fn test_valid_write_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let modified_data = r"a,b,c,d,e\nf,g,h,i,jddd";
        let output_file = write_csv(&modified_data, "output/test.csv");
        assert!(output_file.is_ok());
    }
}