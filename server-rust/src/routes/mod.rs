pub mod menu;
pub mod shopping;
pub mod recipe;

use actix_web::{web, HttpResponse};
use std::sync::Arc;
use crate::config::Config;

pub fn configure(cfg: &mut web::ServiceConfig, config: Arc<Config>) {
    cfg.app_data(web::Data::new(config))
        .service(
            web::scope("/api")
                .route("/menu/generate", web::post().to(menu::generate))
                .route("/menu/recommend", web::get().to(menu::recommend))
                .route("/shopping/generate", web::post().to(shopping::generate))
                .route("/shopping/inventory", web::post().to(shopping::update_inventory))
                .route("/recipe/search", web::get().to(recipe::search))
                .route("/recipe/{name}", web::get().to(recipe::get))
                .route("/health", web::get().to(health)),
        );
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "status": "ok",
            "message": "一日三餐 Rust 服务运行正常",
            "version": "1.0.0"
        }
    }))
}
