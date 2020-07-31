//! 从零开始实现一个完整功能包
//! 使用命令行参数，操作csv文件, 并输出新的csv文件。

use std::path::PathBuf;
use std::process;

/// 增加StructOpt特性的路径 否则from_args()不存在
use structopt::StructOpt;

/// 使用自定义lib中的事务。当在lib内已经use了路径，可以直接使用名称时，外层再次use只需直接使用名称即可。
use t10_3_0_csv_challenge::{load_csv, Opt, replace_column, write_csv};

fn main() {
    // 1 从命令行参数 读取文件
    let opt = Opt::from_args();
    let filename = PathBuf::from(opt.input);
    let csv_data = match load_csv(filename) {
        Ok(fname) => { fname }
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    };

    // 2 替换指定列
    let modified_data = match
        replace_column(csv_data, &opt.column_name, &opt.replacement)
        {
            Ok(data) => { data }
            Err(e) => {
                println!("main error: {:?}", e);
                process::exit(1);
            }
        };

    // 3 写处到指定文件
    let output_file = &opt.output
        .unwrap_or("output/output.csv".to_string());

    match write_csv(&modified_data, &output_file) {
        Ok(_) => {
            println!("write success!");
        }
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    }
}