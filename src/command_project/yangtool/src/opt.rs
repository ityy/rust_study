//! 命令行解析映射文件


use crate::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
name = env ! ("CARGO_PKG_NAME"),
version = env ! ("CARGO_PKG_VERSION"),
author = "yangyang <ofyang@qq.com>",
about = "This is a command line project for quick execution of Linux commands"
)]
pub enum Config {
    /// execution openresty, must default path
    Or {
        #[structopt(short, long)]
        reload: bool,

        #[structopt(short, long)]
        quit: bool,

        #[structopt(short, long)]
        start: bool,
    },

    /// execution java project
    Jar {
        #[structopt(short, long)]
        dev: bool,

        #[structopt(short, long)]
        prod: bool,

        #[structopt(short, long)]
        rename: Option<String>,
    },
}