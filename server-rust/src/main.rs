mod config;
mod error;
mod models;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();

    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = config::Config::from_env();
    tracing::info!("🚀 启动服务器 {}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(config.clone()))
            .configure(routes::configure)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
