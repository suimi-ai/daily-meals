use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub difficulty: u8,
    pub time: String,
    pub servings: u8,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<RecipeStep>,
    pub nutrition: Option<RecipeNutrition>,
    pub tips: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub name: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeStep {
    pub step: u8,
    pub title: String,
    pub description: String,
    pub time: String,
    pub tips: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeNutrition {
    pub calories: u32,
    pub protein: u32,
    pub carbs: u32,
    pub fat: u32,
}
