use crate::error::AppError;
use crate::models::{Dish, Menu, Nutrition};

pub struct MenuService;

impl MenuService {
    pub fn new() -> Self {
        Self
    }

    pub async fn generate_menu(
        &self,
        meal_type: &str,
        _servings: u8,
    ) -> Result<Menu, AppError> {
        // 模拟数据（实际应该调用 AI API）
        let (dishes, nutrition) = match meal_type {
            "早餐" => (
                vec![
                    Dish {
                        name: "鸡蛋灌饼".to_string(),
                        description: "酥脆的饼皮包裹嫩滑鸡蛋".to_string(),
                        difficulty: 2,
                        time: "15分钟".to_string(),
                    },
                    Dish {
                        name: "小米粥".to_string(),
                        description: "养胃暖身，搭配小菜更美味".to_string(),
                        difficulty: 1,
                        time: "30分钟".to_string(),
                    },
                ],
                Nutrition {
                    calories: 450,
                    protein: 15,
                    carbs: 60,
                    fat: 18,
                },
            ),
            "午餐" => (
                vec![
                    Dish {
                        name: "红烧肉".to_string(),
                        description: "肥而不腻，入口即化".to_string(),
                        difficulty: 4,
                        time: "90分钟".to_string(),
                    },
                    Dish {
                        name: "清炒时蔬".to_string(),
                        description: "新鲜蔬菜，清淡爽口".to_string(),
                        difficulty: 2,
                        time: "10分钟".to_string(),
                    },
                ],
                Nutrition {
                    calories: 650,
                    protein: 25,
                    carbs: 45,
                    fat: 35,
                },
            ),
            "晚餐" => (
                vec![
                    Dish {
                        name: "清蒸鲈鱼".to_string(),
                        description: "鲜嫩多汁，保留原味".to_string(),
                        difficulty: 3,
                        time: "25分钟".to_string(),
                    },
                    Dish {
                        name: "蒜蓉西兰花".to_string(),
                        description: "营养健康，色彩诱人".to_string(),
                        difficulty: 2,
                        time: "10分钟".to_string(),
                    },
                ],
                Nutrition {
                    calories: 400,
                    protein: 30,
                    carbs: 25,
                    fat: 15,
                },
            ),
            _ => {
                return Err(AppError::ValidationError(
                    "用餐类型必须是：早餐、午餐或晚餐".to_string(),
                ))
            }
        };

        Ok(Menu { dishes, nutrition })
    }

    pub async fn get_recommendations(&self) -> serde_json::Value {
        serde_json::json!({
            "popular": [
                {"name": "宫保鸡丁", "rating": 4.8, "orders": 1256},
                {"name": "麻婆豆腐", "rating": 4.7, "orders": 987},
            ],
            "seasonal": [
                {"name": "春笋炒肉", "reason": "春季时令"},
            ]
        })
    }
}
