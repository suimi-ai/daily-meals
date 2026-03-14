use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::error::AppError;
use crate::services::MenuService;
use crate::utils::response::{success, success_with_message};

#[derive(Debug, Deserialize)]
pub struct GenerateMenuRequest {
    pub meal_type: String,
    #[serde(default = "default_servings")]
    pub servings: u8,
    #[serde(default)]
    pub preferences: Vec<String>,
    #[serde(default)]
    pub restrictions: Vec<String>,
}

fn default_servings() -> u8 {
    2
}

pub async fn generate(
    service: web::Data<MenuService>,
    body: web::Json<GenerateMenuRequest>,
) -> Result<HttpResponse, AppError> {
    let menu = service
        .generate_menu(&body.meal_type, body.servings)
        .await?;
    Ok(HttpResponse::Ok().json(success_with_message(menu, "菜单生成成功")))
}

pub async fn recommend(service: web::Data<MenuService>) -> Result<HttpResponse, AppError> {
    let recommendations = service.get_recommendations().await;
    Ok(HttpResponse::Ok().json(success(recommendations)))
}
