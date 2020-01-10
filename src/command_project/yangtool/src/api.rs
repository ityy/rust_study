use std::io;
use std::process::{Command, Output};

pub fn or_exec(reload: bool, quit: bool, start: bool) -> io::Result<Output> {
    if reload {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").args(&["-s", "reload"]).output()
    } else if quit {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").arg("quit").output()
    } else {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").output()
    }
}

pub fn jar_exec(dev: bool, prod: bool) -> io::Result<Output> {
    if dev {
        Command::new("cat").arg("./test.txt").output()
    } else if prod {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").arg("quit").output()
    } else {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").output()
    }
}
