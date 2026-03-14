use crate::error::AppError;
use crate::models::{Ingredient, ShoppingList, ShoppingSummary};
use std::collections::HashMap;

pub struct ShoppingService;

impl ShoppingService {
    pub fn new() -> Self {
        Self
    }

    pub async fn generate_list(
        &self,
        dishes: Vec<String>,
        servings: u8,
        existing_ingredients: Vec<String>,
    ) -> Result<ShoppingList, AppError> {
        // 模拟食材数据库
        let ingredient_db = self.get_ingredient_database();

        // 收集所有食材
        let mut all_ingredients: Vec<Ingredient> = Vec::new();

        for dish_name in dishes {
            if let Some(ingredients) = ingredient_db.get(&dish_name) {
                let adjusted = ingredients
                    .iter()
                    .map(|ing| self.adjust_amount(ing.clone(), servings))
                    .collect::<Vec<_>>();
                all_ingredients.extend(adjusted);
            }
        }

        // 合并相同食材
        let merged = self.merge_ingredients(all_ingredients);

        // 标记已有食材
        let final_list = merged
            .into_iter()
            .map(|mut ing| {
                let status = if existing_ingredients.contains(&ing.name) {
                    "已有".to_string()
                } else {
                    "需购买".to_string()
                };
                ing.status = Some(status);
                ing.purchase_tip = Some(self.get_purchase_tip(&ing.category));
                ing.purchased = Some(false);
                ing
            })
            .collect();

        // 分类
        let categorized = self.categorize_ingredients(final_list);

        // 统计
        let total_items: usize = categorized.values().map(|v| v.len()).sum();
        let need_to_buy = categorized
            .values()
            .flat_map(|v| v.iter())
            .filter(|i| i.status.as_deref() == Some("需购买"))
            .count();
        let already_have = total_items - need_to_buy;

        Ok(ShoppingList {
            summary: ShoppingSummary {
                total_items,
                need_to_buy,
                already_have,
            },
            ingredients: categorized,
        })
    }

    fn get_ingredient_database(&self) -> HashMap<String, Vec<Ingredient>> {
        let mut db = HashMap::new();

        db.insert(
            "红烧肉".to_string(),
            vec![
                Ingredient {
                    name: "五花肉".to_string(),
                    amount: "500g".to_string(),
                    category: "肉类".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
                Ingredient {
                    name: "生姜".to_string(),
                    amount: "30g".to_string(),
                    category: "调料".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
                Ingredient {
                    name: "大葱".to_string(),
                    amount: "2根".to_string(),
                    category: "蔬菜".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
                Ingredient {
                    name: "八角".to_string(),
                    amount: "3个".to_string(),
                    category: "调料".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
            ],
        );

        db.insert(
            "清炒时蔬".to_string(),
            vec![
                Ingredient {
                    name: "时令蔬菜".to_string(),
                    amount: "300g".to_string(),
                    category: "蔬菜".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
                Ingredient {
                    name: "大蒜".to_string(),
                    amount: "3瓣".to_string(),
                    category: "调料".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
            ],
        );

        db.insert(
            "番茄蛋汤".to_string(),
            vec![
                Ingredient {
                    name: "番茄".to_string(),
                    amount: "2个".to_string(),
                    category: "蔬菜".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
                Ingredient {
                    name: "鸡蛋".to_string(),
                    amount: "2个".to_string(),
                    category: "蛋类".to_string(),
                    status: None,
                    purchase_tip: None,
                    purchased: None,
                },
            ],
        );

        db
    }

    fn adjust_amount(&self, mut ingredient: Ingredient, servings: u8) -> Ingredient {
        let base_servings = 2u8;
        if servings == base_servings {
            return ingredient;
        }

        let multiplier = servings as f32 / base_servings as f32;

        // 简单的分量调整
        if let Some(num) = ingredient.amount.chars().next() {
            if num.is_numeric() {
                if let Some(pos) = ingredient.amount.find(|c: char| !c.is_numeric()) {
                    if let Ok(original_num) = ingredient.amount[..pos].parse::<u32>() {
                        let adjusted = (original_num as f32 * multiplier) as u32;
                        ingredient.amount = format!("{}{}", adjusted, &ingredient.amount[pos..]);
                    }
                }
            }
        }

        ingredient
    }

    fn merge_ingredients(&self, ingredients: Vec<Ingredient>) -> Vec<Ingredient> {
        let mut merged: HashMap<String, Ingredient> = HashMap::new();

        for ing in ingredients {
            let key = ing.name.clone();
            if let Some(existing) = merged.get_mut(&key) {
                // 简单合并：保留第一个的量
                // 实际项目中需要更复杂的单位换算
            } else {
                merged.insert(key, ing);
            }
        }

        merged.into_values().collect()
    }

    fn get_purchase_tip(&self, category: &str) -> String {
        match category {
            "肉类" => "建议选择新鲜、有光泽的肉品".to_string(),
            "蔬菜" => "选择叶片翠绿、无黄叶的新鲜蔬菜".to_string(),
            "调料" => "可在超市一次性购齐常用调料".to_string(),
            "蛋类" => "检查蛋壳完整，无裂纹".to_string(),
            _ => "根据个人喜好选择".to_string(),
        }
    }

    fn categorize_ingredients(&self, ingredients: Vec<Ingredient>) -> HashMap<String, Vec<Ingredient>> {
        let mut categorized: HashMap<String, Vec<Ingredient>> = HashMap::new();

        for ing in ingredients {
            categorized
                .entry(ing.category.clone())
                .or_insert_with(Vec::new)
                .push(ing);
        }

        categorized
    }
}
