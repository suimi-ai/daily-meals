use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::config::Config;
use crate::error::AppError;
use crate::services::RecipeService;
use crate::utils::response::success;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub keyword: Option<String>,
}

pub async fn get(
    _config: web::Data<Arc<Config>>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let dish_name = path.into_inner();
    let service = RecipeService::new();

    match service.get_recipe(&dish_name).await? {
        Some(recipe) => Ok(HttpResponse::Ok().json(success(recipe))),
        None => Err(AppError::NotFoundError("未找到该菜谱".to_string())),
    }
}

pub async fn search(
    _config: web::Data<Arc<Config>>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    let service = RecipeService::new();
    let recipes = service.search_recipes(query.keyword.as_deref()).await?;

    Ok(HttpResponse::Ok().json(success(recipes)))
}
