#[macro_use]
extern crate log;

use std::io;
use std::io::Error;
use std::process::{Command, Output};

use env_logger::Env;
use structopt::StructOpt;

use api::*;
use opt::Config;

mod opt;
mod api;

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let config = Config::from_args();

    match config {
        Config::Or {
            reload,
            quit,
            start,
        } => {
            let result = or_exec(reload, quit, start);
            result_handle(result, "can't found openresty in your system.");
        }
        Config::Jar {
            dev,
            prod,
        } => {
            let result = jar_exec(dev, prod);
            result_handle(result, "can't found any jar in this path.");
        }
    };
}

fn result_handle(result: Result<Output, Error>, message: &str) {
    match result {
        Ok(t) => {
            if !t.stdout.is_empty() {
                info!("{}", String::from_utf8(t.stdout).unwrap());
            }
            if !t.stderr.is_empty() {
                info!("{}", String::from_utf8(t.stderr).unwrap());
            }
        }
        Err(e) => error!("{}", message),
    }
}

