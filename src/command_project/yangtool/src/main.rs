#[macro_use]
extern crate log;

use std::io;
use std::io::Error;
use std::process::{Command, Output};

use env_logger::Env;
use structopt::StructOpt;

use opt::Config;

mod opt;

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let config = Config::from_args();

    let result = match config {
        Config::Or {
            reload,
            quit,
            start,
        } => or_exec(reload, quit, start),
        Config::Jar {
            dev,
            prod,
        } => jar_exec(dev, prod),
    };

    match result {
        Ok(t) => info!("{:?}", t),
        Err(e) => error!("{:?}", e),
    }
}

fn or_exec(reload: bool, quit: bool, start: bool) -> io::Result<Output> {
    if reload {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").args(&["-s", "reload"]).output()
    } else if quit {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").arg("quit").output()
    } else {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").output()
    }
}

fn jar_exec(dev: bool, prod: bool) -> io::Result<Output> {
    if dev {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").args(&["-s", "reload"]).output()
    } else if prod {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").arg("quit").output()
    } else {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").output()
    }
}
