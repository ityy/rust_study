use structopt::StructOpt;
use structopt_derive::*;

/// 使用structopt包，可以将命令行参数序列化为struct
/// 命令示例：
/// csv_challenge [flags] <Input file> <Column Name> <Replacement Column Name> [Output file]
/// 定义命令行参数与struct的绑定
#[structopt(name = "csv_challenge", about = "An example of StructOpt usage.")]
#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Needed parameter, the first on the command line.
    // #[structopt(short = "v", long = "verbose")]
    // pub verbosity: u64,
    #[structopt(help = "Input file")]
    pub input: String,
    #[structopt(help = "Column Name")]
    pub column_name: String,
    #[structopt(help = "Replacement Column Name")]
    pub replacement: String,

    ///可选参数 使用Option
    #[structopt(help = "Output file, stdout if not present")]
    pub output: Option<String>,
}

