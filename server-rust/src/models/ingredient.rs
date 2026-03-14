use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub amount: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_tip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingList {
    pub summary: ShoppingSummary,
    pub ingredients: std::collections::HashMap<String, Vec<Ingredient>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingSummary {
    pub total_items: usize,
    pub need_to_buy: usize,
    pub already_have: usize,
}
