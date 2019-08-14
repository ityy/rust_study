extern crate iron;
extern crate router;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

//为结构体自动实现两个特性, 由serde库提供自动实现的逻辑
#[derive(Serialize, Deserialize)]
struct Greeting {
    message: String,
}

pub fn main() {
    let port = "localhost:80";

    //创建路由
    let mut router = Router::new();
    //设置路由
    router.get("/", hello_world, "hello_world");
    router.post("/set/*", set_greeting, "set_greeting");

    println!("binding port: {}", port);
    println!("server is running...");

    //使用设置的路由, 启动server
    Iron::new(router).http(port).unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let greeting = Greeting {
        message: "request is ok!".to_string(),
    };
    let payload = serde_json::to_string(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

// Receive a message by POST and play it back.
fn set_greeting(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    //读取body
    request.body.read_to_string(&mut payload);
    println!("path:{}", request.url.to_string());
    let pathV = request.url.path();
    for s in pathV {
        println!("sub path:{}", s);
    }

    //取header
    let a = request.headers.get_raw("userid");
    let s: String = match a {
        Some(s) => String::from_utf8(s[0].clone()).unwrap(),
        None => "".into(),
    };
    println!("userid:{}", s);

    //反序列化到对象
    let request: Greeting = serde_json::from_str(payload.as_str()).unwrap();
    let greeting = Greeting {
        message: "your params is: ".to_string() + request.message.as_str(),
    };
    //序列化为json文本
    let payload = serde_json::to_string(&greeting).unwrap();
    //返回
    Ok(Response::with((status::Ok, payload)))
}