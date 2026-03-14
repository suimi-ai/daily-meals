use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub difficulty: u8,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    pub dishes: Vec<Dish>,
    pub nutrition: MenuNutrition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuNutrition {
    pub calories: u32,
    pub protein: u32,
    pub carbs: u32,
    pub fat: u32,
}
