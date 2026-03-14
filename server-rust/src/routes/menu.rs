use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::config::Config;
use crate::error::AppError;
use crate::services::MenuService;
use crate::utils::response::{success, success_with_message};

#[derive(Debug, Deserialize)]
pub struct GenerateMenuRequest {
    #[serde(rename = "mealType")]
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
    config: web::Data<Arc<Config>>,
    body: web::Json<GenerateMenuRequest>,
) -> Result<HttpResponse, AppError> {
    let service = MenuService::new(
        config.ai_provider.clone(),
        config.glm_api_key.clone(),
        config.openai_api_key.clone(),
    );

    let menu = service
        .generate_menu(
            &body.meal_type,
            body.servings,
            &body.preferences,
            &body.restrictions,
        )
        .await?;

    Ok(HttpResponse::Ok().json(success_with_message(menu, "菜单生成成功")))
}

pub async fn recommend() -> Result<HttpResponse, AppError> {
    let recommendations = MenuService::get_recommendations().await;
    Ok(HttpResponse::Ok().json(success(recommendations)))
}
