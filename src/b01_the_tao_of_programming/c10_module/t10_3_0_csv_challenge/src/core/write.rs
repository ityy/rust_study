use super::*;

pub fn replace_column(data: String, column: &str, replacement: &str) -> Result<String, Error> {
    let mut lines = data.lines();
    let headers = lines.next().unwrap();
    let columns: Vec<&str> = headers.split(',').collect();
    let column_number = columns.iter().position(|&e| e == column);
    let column_number = match column_number {
        Some(column) => column,
        None => Err("column name doesn’t exist in the input file")?
    };
    let mut result = String::with_capacity(data.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');
    for line in lines {
        /// 行转为Vec，按指定索引替换Vec的元素，再转回为行，完成指定字段的替换。
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::read::load_csv;
    use super::replace_column;

    #[test]
    fn test_valid_replace_column() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing");
        assert!(modified_data.is_ok());
    }

    #[test]
    fn test_invalid_replace_column() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City2", "Beijing");
        assert!(modified_data.is_err());
    }
}