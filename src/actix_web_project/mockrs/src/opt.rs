//! 使用StructOpt从命令行获取参数
//! 配合dotenv库相当于从.env文件获取配置。
//! StructOpt用法：
//!     appName [SubCommand] [-p | --parameter] [value]
//!     使用结构体，则为Command模式。appName为主命令，后方可选flag，可选value
//!     使用枚举，则为SubCommand模式，每个枚举值都是一个子命令。appName为主命令，后方必选SubCommand，可选flag，可选value
//!     参数序列按定义顺序有序。
//! 以字段名parameter举例：
//!     short           表示短flag，以-p开头，后跟value
//!     long            表示长flag，以--parameter开头，后跟value
//!     无short、long     表示该命令为ARGS模式，无需给flag，按位置直接给value。
//!     default_value    给予默认值
//!     env              从环境变量取值
//!     ///              字段的文档注释，会在--help中体现为帮助说明。
//!     Option类型        表示该命令可选
//!     bool类型          表示该命令为flag模式，无需给value，输入则为true，无输入则为false
//!
use crate::StructOpt;

/// 要在 RUST 程序中获得Cargo中的一些值，请执行以下操作:
/// let version = env!("CARGO_PKG_VERSION");
#[derive(StructOpt, Debug, Clone)]
/// 添加一些元信息:
#[structopt(
name = env ! ("CARGO_PKG_NAME"),
version = env ! ("CARGO_PKG_VERSION"),
//author = "PrivateRookie <996514515@qq.com>",
author = env ! ("CARGO_PKG_AUTHORS"),
about = env ! ("CARGO_PKG_DESCRIPTION")
)]
pub enum Config {
    /// Run http json server
    Serve {
        /// Json file as database
        #[structopt(required = true, env = "MOCKRS_DB_FILE")]
        db_file: String,

        /// Listen ip
        #[structopt(long, default_value = "127.0.0.1", env = "MOCKRS_HOST")]
        host: String,

        /// Listen port
        #[structopt(short, long, default_value = "9000", env = "MOCKRS_PORT")]
        port: usize,
    },

    /// Generate fake data based on template
    Gen {
        /// Template file to generate json file
        #[structopt(required = true)]
        template: String,

        /// Output json file
        #[structopt(long)]
        output: Option<String>,
    },
}