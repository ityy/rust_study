use std::fs;
use std::io;
use std::process::{Command, Output};

use chrono::prelude::*;

pub fn or_exec(reload: bool, quit: bool, start: bool) -> io::Result<Output> {
    if reload {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").args(&["-s", "reload"]).output()
    } else if quit {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").arg("quit").output()
    } else {
        Command::new("/usr/local/openresty/nginx/sbin/nginx").output()
    }
}

pub fn jar_exec(dev: bool, prod: bool, rename: Option<String>) -> io::Result<Output> {
    if dev {
        return Command::new("/bin/bash").arg("hello.sh").output();
    } else if prod {
        return Command::new("/usr/local/openresty/nginx/sbin/nginx").arg("quit").output();
    } else if let Some(new_file_name) = rename {
        let paths = fs::read_dir("./").unwrap();
        for path in paths {
            let file_name = path.unwrap().file_name().into_string().unwrap();
            if file_name.ends_with(".jar") {
                let local: DateTime<Local> = Local::now();
                let back_name = format!("{}.{}", file_name, local.format("%Y-%m-%d_%H:%M:%S").to_string());
                Command::new("mv").arg(&file_name).arg(&back_name).output().unwrap();
                Command::new("mv").arg(&new_file_name).arg(&file_name).output().unwrap();
                println!("file: {} change to: {}", file_name, back_name);
                println!("file: {} change to: {}", new_file_name, file_name);
                break;
            }
        }
    };

    Command::new("/bin/bash").arg("hello.sh").output()
}
