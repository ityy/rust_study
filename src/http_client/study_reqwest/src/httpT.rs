extern crate reqwest;

use std::collections::HashMap;

#[test]
pub fn test() {
    h2();
}

fn h1() -> Result<(), Box<dyn std::error::Error>> {
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;
    println!("{:#?}", resp);
    Ok(())
}

fn h2() -> Result<(), Box<dyn std::error::Error>> {
    let r = reqwest::get("https://www.baidu.com/")?
        .text()?;
    println!("{:#?}", r);
    Ok(())
}