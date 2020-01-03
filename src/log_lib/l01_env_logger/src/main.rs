//! 日志框架使用
//!
//! 依赖：
//! [dependencies]
//! log = "0.4.0"
//! env_logger = "0.7.1"
//!
//! 1 尽可能早的初始化日志框架。
//! 2 启动程序前，设置环境变量，以配置日志等级：
//!     $ RUST_LOG=info ./main
//! 3 可以通过环境变量之外的其它方式配置：


// 带宏的包，要显式声明在项目root文件内。
#[macro_use]
extern crate log;

use env_logger::{Builder, Env, fmt};

/// 从环境变量创建logger
#[test]
fn config_from_env() {
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    // Env用来读取环境变量，在这里如果从系统环境变量读不到这两个变量，则按给定值返回它们。
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    // 从对象获取环境变量
    env_logger::init_from_env(env);

    // 打印日志
    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");
}


/// 配置更多日志格式
fn init_logger() {
    //设置环境
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    //创建构造器
    let mut builder = Builder::from_env(env);

    //配置构造器
    builder
        //是否写
        .format_level(false)
        //时间精度
        .format_timestamp_millis()
        //是否写
        .format_module_path(true);

    //初始化logger
    builder.init();
}


#[test]
fn config_from_init_logger() {
    init_logger();
    info!("a log from `MyLogger`");
}


/// 用代码配置logger
#[test]
fn config_from_code() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .write_style(fmt::WriteStyle::Always)
        .init();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");
}

/// 自定义日志格式
#[test]
//#[cfg(all(feature = "termcolor", feature = "humantime"))]
fn test_custom_format() {
    use env_logger::{fmt, Builder, Env};
    use std::io::Write;

    fn init_logger() {
        let env = Env::default()
            .filter_or("MY_LOG_LEVEL", "trace")
            .write_style_or("MY_LOG_STYLE", "always");

        Builder::from_env(env)
            .format(|buf, record| {
                let mut style = buf.style();
                style.set_bg(fmt::Color::Blue).set_bold(true);

                let timestamp = buf.timestamp();

                writeln!(
                    buf,
                    "My formatted log ({}): {}",
                    timestamp,
                    style.value(record.args())
                )
            })
            .init();
    }

    init_logger();

    log::info!("a log from `MyLogger`");
}