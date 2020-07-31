//! Rust也支持基准测试。
//! 在src同级的目录内创建benches目录，cargo会自动将其识别为基准测试目录。
//! 使用 cargo bench 即可自动执行基准测试。
//!
//! 具有集成测试，基准测试，这样完整功能的lib包，即可发布到crates.io上了。

/// 基准测试需要使用#![feature(test)]，而feature功能必须在夜版Rust环境下。
#![feature(test)]
extern crate test;

use std::path::PathBuf;
use test::Bencher;

use t10_3_0_csv_challenge::{
    {load_csv, write_csv},
    Opt,
    replace_column,
};

#[bench]
fn bench_read_100times(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100);
        (0..n).fold(0, |_, _| {
            test_load_csv();
            0
        })
    });
}

fn test_load_csv() {
    let filename = PathBuf::from("./input/challenge.csv");
    load_csv(filename);
}

#[bench]
fn bench_rw_100times(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100);
        (0..n).fold(0, |_, _| {
            test_read_write_csv();
            0
        })
    });
}

fn test_read_write_csv() {
    let filename = PathBuf::from("./input/challenge.csv");
    let csv_data = load_csv(filename).unwrap();
    let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
    write_csv(&modified_data, "output/test.csv");
}
