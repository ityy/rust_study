//! 在动态库中使用第三方依赖
extern crate reqwest;

use std::collections::HashMap;

#[no_mangle]
pub extern fn reqwest_test() -> u32 {
    http_get("https://www.baidu.com/");
    0
}

fn http_get(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(addr)?.text()?;
    println!("http_get is ok, resp: \n {:#?}", resp);
    Ok(())
}