//extern crate iron;
//extern crate serde_json;

use iron::prelude::*;
use iron::status;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

pub fn main() {
    t2();
}


//基础演示
fn t1() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}

//返回json演示
fn t2() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = serde_json::to_string(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}

//为结构体自动实现两个特性, 由serde库提供自动实现的逻辑
#[derive(Serialize, Deserialize)]
struct Greeting {
    msg: String
}