//! Project Github:https://github.com/PrivateRookie/mockrs
#[macro_use]
extern crate log;

use std::io::{Error, ErrorKind, prelude::*};

use actix_web::{App, HttpServer, middleware, web};
use jen::generator::Generator;
use structopt::StructOpt;

use dotenv::dotenv;
use opt::Config;

mod api;
mod db;
mod opt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 读取.env配置文件到本程序的环境变量中
    dotenv().ok();
    // 从取环境变量初始化日志
    env_logger::init();

    //#[derive(StructOpt)] 为Config实现了StructOpt特性 其含有from_args()方法
    let config = Config::from_args();
    info!("start running...");

    // 判断输入参数
    match config {
        Config::Serve {
            db_file,
            host,
            port,
        } => run_server(db_file, host, port).await,
        Config::Gen {
            template,
            output
        } => generate_by_template(template, output),
    }
}

/// actix-web 配置
/// 异步方法
async fn run_server(db_file: String, host: String, port: usize) -> std::io::Result<()> {
    // 创建Database
    let db = db::Database::new(&db_file);
    // 放入为共享数据 web_data为arc包装
    let web_db = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
            // 设置共享数据
            .app_data(web_db.clone())
            // 设置日志
            .wrap(middleware::Logger::default())
            .service(web::resource("/index").route(web::get().to(api::server_info)))
            .service(web::scope("/_actions").route("/flush", web::post().to(api::flush)))
            .service(
                web::resource("/*")
                    .route(web::get().to(api::do_get))
                    .route(web::post().to(api::do_post))
                    .route(web::put().to(api::do_post))
                    .route(web::delete().to(api::do_delete)),
            )
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

/// 根据模板生成数据
fn generate_by_template(template: String, output: Option<String>) -> std::io::Result<()> {
    match Generator::new(template) {
        Err(_) => Err(Error::new(ErrorKind::NotFound, "can not find template")),
        Ok(mut gen) => {
            match output {
                None => println!("{}", gen.create()),
                Some(output) => {
                    let mut f = std::fs::File::create(output)?;
                    f.write(gen.create().as_bytes())?;
                }
            }
            Ok(())
        }
    }
}