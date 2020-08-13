//! # 写操作模块


use super::*;

/// # 写出到文件
/// Usage:
/// ```ignore
/// let filename = PathBuf::from("./files/challenge.csv");
/// let csv_data = load_csv(filename).unwrap();
/// let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
/// let output_file = write_csv(&modified_data, "output/test.csv");
/// assert!(output_file.is_ok());
/// ```
pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error> {
    write(csv_data, filename)?;
    Ok(())
}


fn write(data: &str, filename: &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}


/// # 替换 CSV 中的列
pub fn replace_column(text: String, column: &str, replacement: &str) -> Result<String, Error> {
    let mut lines = text.lines();
    let headers = lines.next().unwrap();
    let columns: Vec<&str> = headers.split(',').collect();

    // 获取目标列的下标序号
    let column_number = columns.iter().position(|&e| e == column);
    let column_number = match column_number {
        Some(column) => column,
        None => Err("column name doesn’t exist in the input file")?, // ? 操作符 当其左边的值为Err类型时，直接抛出
    };

    // 新建String，作为返回值
    let mut result = String::with_capacity(text.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');

    // 替换每一行中，目标列的值
    for line in lines {
        // 行转为Vec，按指定索引替换Vec的元素，再转回为行，完成指定字段的替换。
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }

    Ok(result)
}


/// # 单元测试示例
/// '#[cfg(xxx)]' 仅在执行cargo xxx命令时才会编译执行的代码
mod test {
    use std::path::PathBuf;

    use super::{replace_column, write_csv};
    use super::read::load_csv;

    /// # 测试有效的替换
    #[test]
    fn test_valid_replace_column() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing");
        assert!(modified_data.is_ok());
    }

    /// # 测试无效的替换
    #[test]
    fn test_invalid_replace_column() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City2", "Beijing");
        assert!(modified_data.is_err());
    }


    /// # 测试有效的写文件
    #[test]
    fn test_valid_write_csv() {
        let csv_text = r"1,2,3,4,5
a,b,c,d,e";
        let output_file = write_csv(&csv_text, "output/test.csv");
        assert!(output_file.is_ok());
    }
}