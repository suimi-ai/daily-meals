mod ai;
mod config;
mod error;
mod models;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
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
    let config = Arc::new(config::Config::from_env());
    let host = config.host.clone();
    let port = config.port;
    
    tracing::info!("🚀 启动服务器 {}:{}", host, port);
    tracing::info!("🤖 AI 提供商: {}", config.ai_provider);

    HttpServer::new(move || {
        let config_clone = config.clone();
        App::new()
            .wrap(Cors::permissive())
            .configure(move |cfg| routes::configure(cfg, config_clone.clone()))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
