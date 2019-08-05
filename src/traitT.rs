use std::fmt::Display;

///特性 学习代码
pub fn mainT() {
    //因为整型实现了Display特性, 所以其也实现了DisYang特性
    3.dis_yang();
    "测试".dis_yang();
    //打印结果:
    // this is yang trait!
    // this is yang trait!
}

///自定义特性
trait DisYang {
    //直接给出默认实现
    fn dis_yang(&self) {
        println!("this is yang trait!");
    }
}

///给所有实现了Display特性的对象 也实现DisYang特性
impl<T: Display> DisYang for T {}
