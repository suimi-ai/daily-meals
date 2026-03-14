use crate::error::AppError;
use crate::models::{Recipe, RecipeIngredient, RecipeStep};
use serde_json::Value;

pub struct RecipeService;

impl RecipeService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_recipe(&self, dish_name: &str) -> Result<Option<Recipe>, AppError> {
        // 模拟菜谱数据库
        let recipe = self.get_mock_recipe(dish_name);
        Ok(recipe)
    }

    pub async fn search_recipes(&self, keyword: Option<&str>) -> Result<Vec<Value>, AppError> {
        let all_recipes = vec![
            serde_json::json!({
                "name": "红烧肉",
                "description": "经典家常菜，肥而不腻",
                "difficulty": 4,
                "time": "90分钟"
            }),
            serde_json::json!({
                "name": "清蒸鲈鱼",
                "description": "鲜嫩多汁，保留原味",
                "difficulty": 3,
                "time": "25分钟"
            }),
            serde_json::json!({
                "name": "宫保鸡丁",
                "description": "酸甜微辣，经典川菜",
                "difficulty": 3,
                "time": "30分钟"
            }),
        ];

        if let Some(kw) = keyword {
            let filtered: Vec<Value> = all_recipes
                .into_iter()
                .filter(|r| {
                    r["name"].as_str().unwrap_or("").contains(kw)
                        || r["description"].as_str().unwrap_or("").contains(kw)
                })
                .collect();
            Ok(filtered)
        } else {
            Ok(all_recipes)
        }
    }

    fn get_mock_recipe(&self, dish_name: &str) -> Option<Recipe> {
        match dish_name {
            "红烧肉" => Some(Recipe {
                name: "红烧肉".to_string(),
                description: "经典家常菜，肥而不腻，入口即化".to_string(),
                difficulty: 4,
                time: "90分钟".to_string(),
                servings: 4,
                ingredients: vec![
                    RecipeIngredient {
                        name: "五花肉".to_string(),
                        amount: "500g".to_string(),
                    },
                    RecipeIngredient {
                        name: "生姜".to_string(),
                        amount: "30g".to_string(),
                    },
                    RecipeIngredient {
                        name: "大葱".to_string(),
                        amount: "2根".to_string(),
                    },
                ],
                steps: vec![
                    RecipeStep {
                        step: 1,
                        title: "准备食材".to_string(),
                        description: "五花肉切成3-4cm见方的块，生姜切片，大葱切段".to_string(),
                        time: "10分钟".to_string(),
                        tips: Some("选择肥瘦相间的五花肉，口感最佳".to_string()),
                    },
                    RecipeStep {
                        step: 2,
                        title: "焯水去腥".to_string(),
                        description: "肉块冷水下锅，加入料酒和姜片，煮沸后撇去浮沫，捞出沥干".to_string(),
                        time: "10分钟".to_string(),
                        tips: Some("焯水可以去除血水和腥味".to_string()),
                    },
                    RecipeStep {
                        step: 3,
                        title: "炒糖色".to_string(),
                        description: "锅中放少许油，加入冰糖，小火炒至焦糖色".to_string(),
                        time: "5分钟".to_string(),
                        tips: Some("糖色不能炒过，否则会发苦".to_string()),
                    },
                ],
                nutrition: Some(crate::models::RecipeNutrition {
                    calories: 520,
                    protein: 25,
                    carbs: 15,
                    fat: 42,
                }),
                tips: Some(vec![
                    "选用五花肉要肥瘦相间，这样烧出来才好吃".to_string(),
                    "炒糖色是关键，决定了红烧肉的颜色和口感".to_string(),
                ]),
            }),
            "清蒸鲈鱼" => Some(Recipe {
                name: "清蒸鲈鱼".to_string(),
                description: "鲜嫩多汁，保留原味".to_string(),
                difficulty: 3,
                time: "25分钟".to_string(),
                servings: 2,
                ingredients: vec![
                    RecipeIngredient {
                        name: "鲈鱼".to_string(),
                        amount: "1条".to_string(),
                    },
                    RecipeIngredient {
                        name: "生姜".to_string(),
                        amount: "20g".to_string(),
                    },
                    RecipeIngredient {
                        name: "大葱".to_string(),
                        amount: "1根".to_string(),
                    },
                ],
                steps: vec![
                    RecipeStep {
                        step: 1,
                        title: "处理鱼".to_string(),
                        description: "鲈鱼清洗干净，在鱼身两侧划几刀".to_string(),
                        time: "5分钟".to_string(),
                        tips: Some("划刀可以帮助入味".to_string()),
                    },
                    RecipeStep {
                        step: 2,
                        title: "腌制".to_string(),
                        description: "用料酒、盐、姜片腌制10分钟".to_string(),
                        time: "10分钟".to_string(),
                        tips: None,
                    },
                    RecipeStep {
                        step: 3,
                        title: "蒸制".to_string(),
                        description: "水开后放入鱼，大火蒸8-10分钟".to_string(),
                        time: "10分钟".to_string(),
                        tips: Some("不要蒸过头，影响口感".to_string()),
                    },
                ],
                nutrition: Some(crate::models::RecipeNutrition {
                    calories: 180,
                    protein: 30,
                    carbs: 5,
                    fat: 6,
                }),
                tips: Some(vec![
                    "选择新鲜的鲈鱼，肉质更鲜美".to_string(),
                ]),
            }),
            _ => None,
        }
    }
}
