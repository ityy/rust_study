//! # 设计模式
//! 设计模式思想涵盖了下面4个点：
//! -   针对接口编程
//! -   组合优于继承
//! -   分离变和不变
//! -   委托代替继承
//!
//! 可以说Rust本身的设计就非常符合这4点思想：
//! -   trait可以强制性地实现针对接口编程
//! -   泛型和trait限定可以替代继承实现多态，基于代数数据类型的结构体和枚举体在没有继承的情况下也一样可以更自由的构造各种类型
//! -   类型系统天生分离了变和不变
//! -   常用的迭代器就是利用委托来代替继承的
//!
//! Rust是一门已经自举的语言，内部也用到了很多设计模式，比如迭代器适配器就包含了委托模式和迭代器模式的思想。

use std::process::Command;

/// # 建造者模式
/// Rust最常用的模式之一。Rust结构体没有构造函数，不像java提供默认构造函数。也没有默认值，不像java会将值初始化为0。<br/>
/// 建造者模式是指使用多个简单对象一步步构建一个复杂对象的模式。该模式的主要思想是将变和不变分离。一个复杂对象肯定有变的和不变的部分，进行分离构建。
#[test]
fn test_builder() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        /// 求面积
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        /// 返回一个建造者
        fn new() -> CircleBuilder {
            CircleBuilder {
                x: 0.0,
                y: 0.0,
                radius: 0.0,
            }
        }
    }

    /// 构造器 分块构造目标对象
    impl CircleBuilder {
        fn x(&mut self, x: f64) -> &mut CircleBuilder {
            self.x = x;
            self
        }
        fn y(&mut self, y: f64) -> &mut CircleBuilder {
            self.y = y;
            self
        }
        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
            self.radius = radius;
            self
        }
        fn build(&self) -> Circle {
            Circle {
                x: self.x,
                y: self.y,
                radius: self.radius,
            }
        }
    }

    //可以用非常优雅的链式调用创建对象了
    let c = Circle::new().x(1.0).y(2.0).radius(2.0).build();
    println!("{}", c.area());
    println!("{}", c.x);
    println!("{}", c.y);

    //Rust标准库中有一个用于创建进程的结构体，也使用了创造者模式
    Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
}


/// # 访问者模式
/// 访问者模式用于将数据结构和作用于结构上的操作解耦。即数据结构在calss或struct上，操作在interface或trait上。<br/>
/// 访问者模式一般包含两个层次：
/// -   定义需要操作的元素
/// -   定义相关的操作
///
/// ## Serde
/// 访问者模式的一个经典案例就是第三方库Serde，这是一个高效的序列化/反序列化框架，名字也是由Serialize和Deserialize的前几个字母拼接而成。<br/>
/// Serde仅定义了统一的数据模型，并通过访问者模式开放了序列化和反序列化的接口。Serde目前已经支持了很多种数据格式，包括JSON、XML、YAML、TOML等。
fn nothing() {}

/// # RAII模式 （Resource Acquisition Is Initialization）
/// 也称为“资源获取就是初始化”，是c++等编程语言常用的管理资源、避免内存泄露的方法。它保证在任何情况下，使用对象时先构造对象，最后析构对象。<br/>
/// Rust的一大特性就是使用RAII进行资源管理，让我们能够编写安全的代码。
///
/// # 演示RAII：无 RAII 的 test_raii_1
#[test]
fn test_raii_1() {
    #[derive(Clone)]
    //信
    pub struct Letter {
        text: String,
    }
    //信封
    pub struct Envelope {
        letter: Option<Letter>
    }
    //卡车
    pub struct PickupLorryHandle {
        done: bool,
    }
    impl Letter {
        pub fn new(text: String) -> Self {
            Letter { text }
        }
    }
    impl Envelope {
        pub fn wrap(&mut self, letter: &Letter) {
            self.letter = Some(letter.clone())
        }
    }
    impl PickupLorryHandle {
        /// 装车
        pub fn pickup(&mut self, _envelope: &Envelope) {
            //give letter
        }
        /// 寄送
        pub fn done(&mut self) {
            self.done = true;
            println!("sent");
        }
    }

    /// 购买一个信封（已贴邮票的）
    pub fn buy_restamped_envelope() -> Envelope {
        Envelope { letter: None }
    }

    /// 下单准备寄送
    pub fn order_pickup() -> PickupLorryHandle {
        PickupLorryHandle {
            done: false,
        }
    }

    //测试
    let letter = Letter::new("Dear RustFest".to_string());
    let mut envelope = buy_restamped_envelope();
    envelope.wrap(&letter);
    let mut lorry = order_pickup();
    lorry.pickup(&envelope);
    lorry.done();
}

/// # 演示RAII：使用 RAII 改进 test_raii_1
/// test_raii_1()的逻辑已经完成，初看好像没有什么逻辑问题。还是存在以下问题：
/// -   Letter有可能被复制多份并被装到多个信封中，不安全
/// -   Envelope里可能有信，也可能无信，不安全
/// -   无法保证一定把信交给邮车
/// 为了解决这些问题，可以使用RAII模式来重构test_raii_1()的逻辑。
#[test]
fn test_raii_2() {
    //信
    pub struct Letter {
        text: String,
    }

    //空信封
    pub struct EmptyEnvelope {}
    //封好的信封
    pub struct ClosedEnvelope {
        letter: Letter
    }
    //卡车
    pub struct PickupLorryHandle {
        done: bool,
    }
    impl Letter {
        pub fn new(text: String) -> Self {
            Letter { text }
        }
    }
    impl EmptyEnvelope {
        pub fn wrap(self, letter: Letter) -> ClosedEnvelope {
            ClosedEnvelope {
                letter
            }
        }
    }

    impl PickupLorryHandle {
        /// 装车
        pub fn pickup(&mut self, _envelope: ClosedEnvelope) {
            //give letter
        }
    }

    /// 发车由drop触发，发车完即销毁回收
    impl Drop for PickupLorryHandle {
        fn drop(&mut self) {
            println!("sent");
        }
    }

    /// 购买一个信封（已贴邮票的）
    pub fn buy_restamped_envelope() -> EmptyEnvelope {
        EmptyEnvelope {}
    }

    /// 下单准备寄送
    pub fn order_pickup() -> PickupLorryHandle {
        PickupLorryHandle {
            done: false,
        }
    }

    //测试
    let letter = Letter::new("Dear RustFest".to_string());
    let envelope = buy_restamped_envelope();//确保买到的是空信封
    let closed_envelope = envelope.wrap(letter);//转移所有权，确保信件只能被装入一个信封，确保得到的是已装信的信封
    let mut lorry = order_pickup();
    lorry.pickup(closed_envelope);//转移所有权，确保装车的是已装信的信封

    /// main结束后自动运行了drop方法，这正是RAII的体现，不仅释放了资源，也在逻辑上保证了信件被寄出。
    /// 所以，所谓RAII模式，并非经典的GOF中的模式，它实际上是利用Rust的RAII机制来确保逻辑安全的一种模式。
    /// 这种模式在某些场景中非常适用，比如处理HTTP请求的场景，它也是Rust官方推荐使用的模式。
    ///
    /// Rust语言的哲学是组合优于继承，结构体，枚举体，trait，就是一个个榫卯（sun mao），可以自由组合出想要的结构。
    /// 本章介绍了三种设计模式，在Rust中被大量使用。
    fn nothing() {}
}
