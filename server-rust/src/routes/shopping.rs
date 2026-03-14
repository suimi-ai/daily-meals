use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::config::Config;
use crate::error::AppError;
use crate::services::ShoppingService;
use crate::utils::response::success;

#[derive(Debug, Deserialize)]
pub struct GenerateListRequest {
    pub dishes: Vec<DishInput>,
    #[serde(default = "default_servings")]
    pub servings: u8,
    #[serde(default)]
    pub existing_ingredients: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DishInput {
    pub name: String,
}

fn default_servings() -> u8 {
    2
}

pub async fn generate(
    _config: web::Data<Arc<Config>>,
    body: web::Json<GenerateListRequest>,
) -> Result<HttpResponse, AppError> {
    let service = ShoppingService::new();

    let dish_names: Vec<String> = body.dishes.iter().map(|d| d.name.clone()).collect();

    let list = service
        .generate_list(dish_names, body.servings, body.existing_ingredients.clone())
        .await?;

    Ok(HttpResponse::Ok().json(success(list)))
}

pub async fn update_inventory() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(success(serde_json::json!({
        "updated": true,
        "count": 0,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))))
}
