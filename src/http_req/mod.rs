use http_req::request;

pub fn main() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("https://www.baidu.com/", &mut writer).unwrap();
    let ws: String = String::from_utf8(writer).unwrap();
    println!("Status: {} {}", res.status_code(), res.reason());
    println!("{:#?}", res);
    println!("{:#?}", ws);
}