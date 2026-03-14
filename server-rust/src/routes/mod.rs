pub mod menu;

use actix_web::{web, HttpResponse};
use std::sync::Arc;
use crate::config::Config;

pub fn configure(cfg: &mut web::ServiceConfig, config: Arc<Config>) {
    cfg.app_data(web::Data::new(config))
        .service(
            web::scope("/api")
                .route("/menu/generate", web::post().to(menu::generate))
                .route("/menu/recommend", web::get().to(menu::recommend))
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
