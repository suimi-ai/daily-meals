pub mod menu;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/menu/generate", web::post().to(menu::generate))
            .route("/menu/recommend", web::get().to(menu::recommend))
            // TODO: 添加其他路由
            .route("/health", web::get().to(health)),
    );
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "status": "ok",
            "message": "一日三餐 Rust 服务运行正常"
        }
    }))
}
