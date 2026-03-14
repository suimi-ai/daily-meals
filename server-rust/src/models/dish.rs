use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub difficulty: u8,
    pub time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meal_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servings: Option<u8>,
    pub dishes: Vec<Dish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nutrition: Option<MenuNutrition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuNutrition {
    pub total_calories: u32,
    pub protein: u32,
    pub carbs: u32,
    pub fat: u32,
}
