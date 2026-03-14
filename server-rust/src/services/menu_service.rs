use crate::error::AppError;
use crate::models::Menu;
use reqwest::Client;
use serde_json::json;

pub struct MenuService {
    client: Client,
    ai_provider: String,
    glm_api_key: Option<String>,
    openai_api_key: Option<String>,
}

impl MenuService {
    pub fn new(
        ai_provider: String,
        glm_api_key: Option<String>,
        openai_api_key: Option<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            ai_provider,
            glm_api_key,
            openai_api_key,
        }
    }

    pub async fn generate_menu(
        &self,
        meal_type: &str,
        servings: u8,
        preferences: &[String],
        restrictions: &[String],
    ) -> Result<Menu, AppError> {
        // 尝试使用 AI 生成
        if let Some(menu) = self
            .try_ai_generation(meal_type, servings, preferences, restrictions)
            .await?
        {
            return Ok(menu);
        }

        // 降级到模拟数据
        Ok(self.get_mock_menu(meal_type, servings))
    }

    async fn try_ai_generation(
        &self,
        meal_type: &str,
        servings: u8,
        preferences: &[String],
        restrictions: &[String],
    ) -> Result<Option<Menu>, AppError> {
        match self.ai_provider.as_str() {
            "glm" => {
                if let Some(api_key) = &self.glm_api_key {
                    match self
                        .call_glm_api(meal_type, servings, preferences, restrictions, api_key)
                        .await
                    {
                        Ok(menu) => Ok(Some(menu)),
                        Err(e) => {
                            tracing::warn!("GLM API 调用失败: {}, 使用降级方案", e);
                            Ok(None)
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            "openai" => {
                if let Some(api_key) = &self.openai_api_key {
                    match self
                        .call_openai_api(meal_type, servings, preferences, restrictions, api_key)
                        .await
                    {
                        Ok(menu) => Ok(Some(menu)),
                        Err(e) => {
                            tracing::warn!("OpenAI API 调用失败: {}, 使用降级方案", e);
                            Ok(None)
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    async fn call_glm_api(
        &self,
        meal_type: &str,
        servings: u8,
        preferences: &[String],
        restrictions: &[String],
        api_key: &str,
    ) -> Result<Menu, AppError> {
        let prompt = format!(
            "请为{}生成{}人份的菜单。饮食偏好：{}。饮食限制：{}。",
            meal_type,
            servings,
            preferences.join("、"),
            restrictions.join("、")
        );

        let _response = self
            .client
            .post("https://open.bigmodel.cn/api/paas/v3/model-api/chatglm_pro/invoke")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "prompt": prompt,
                "temperature": 0.7,
            }))
            .send()
            .await
            .map_err(|e| AppError::AIServiceError(format!("GLM API 请求失败: {}", e)))?;

        // TODO: 实现完整的解析逻辑
        Err(AppError::AIServiceError(
            "GLM 响应解析未完成".to_string(),
        ))
    }

    async fn call_openai_api(
        &self,
        meal_type: &str,
        servings: u8,
        preferences: &[String],
        restrictions: &[String],
        api_key: &str,
    ) -> Result<Menu, AppError> {
        let prompt = format!(
            "请为{}生成{}人份的菜单。饮食偏好：{}。饮食限制：{}。返回 JSON 格式。",
            meal_type,
            servings,
            preferences.join("、"),
            restrictions.join("、")
        );

        let _response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {"role": "system", "content": "你是一个专业的营养师和厨师助手。"},
                    {"role": "user", "content": prompt}
                ],
                "temperature": 0.7,
            }))
            .send()
            .await
            .map_err(|e| AppError::AIServiceError(format!("OpenAI API 请求失败: {}", e)))?;

        // TODO: 实现完整的解析逻辑
        Err(AppError::AIServiceError(
            "OpenAI 响应解析未完成".to_string(),
        ))
    }

    fn get_mock_menu(&self, meal_type: &str, servings: u8) -> Menu {
        let dishes = match meal_type {
            "早餐" => vec![
                crate::models::Dish {
                    name: "小米粥".to_string(),
                    description: "养胃健脾，营养丰富".to_string(),
                    difficulty: 1,
                    time: "30分钟".to_string(),
                    calories: Some(150 * servings as u32),
                },
                crate::models::Dish {
                    name: "鸡蛋饼".to_string(),
                    description: "简单快手，美味可口".to_string(),
                    difficulty: 2,
                    time: "15分钟".to_string(),
                    calories: Some(200 * servings as u32),
                },
            ],
            "午餐" => vec![
                crate::models::Dish {
                    name: "红烧肉".to_string(),
                    description: "经典家常菜，肥而不腻".to_string(),
                    difficulty: 4,
                    time: "90分钟".to_string(),
                    calories: Some(520 * servings as u32),
                },
                crate::models::Dish {
                    name: "清炒时蔬".to_string(),
                    description: "清淡爽口，营养健康".to_string(),
                    difficulty: 2,
                    time: "10分钟".to_string(),
                    calories: Some(80 * servings as u32),
                },
            ],
            "晚餐" => vec![
                crate::models::Dish {
                    name: "清蒸鲈鱼".to_string(),
                    description: "鲜嫩多汁，保留原味".to_string(),
                    difficulty: 3,
                    time: "25分钟".to_string(),
                    calories: Some(180 * servings as u32),
                },
                crate::models::Dish {
                    name: "番茄蛋汤".to_string(),
                    description: "酸甜可口，开胃消食".to_string(),
                    difficulty: 1,
                    time: "15分钟".to_string(),
                    calories: Some(120 * servings as u32),
                },
            ],
            _ => vec![crate::models::Dish {
                name: "营养套餐".to_string(),
                description: "均衡营养，健康美味".to_string(),
                difficulty: 2,
                time: "30分钟".to_string(),
                calories: Some(300 * servings as u32),
            }],
        };

        Menu {
            meal_type: Some(meal_type.to_string()),
            servings: Some(servings),
            dishes,
            nutrition: Some(crate::models::MenuNutrition {
                total_calories: 800 * servings as u32,
                protein: 35 * servings as u32,
                carbs: 100 * servings as u32,
                fat: 25 * servings as u32,
            }),
        }
    }

    pub async fn get_recommendations() -> Menu {
        Menu {
            meal_type: Some("推荐".to_string()),
            servings: Some(2),
            dishes: vec![
                crate::models::Dish {
                    name: "红烧肉".to_string(),
                    description: "经典家常菜，肥而不腻".to_string(),
                    difficulty: 4,
                    time: "90分钟".to_string(),
                    calories: Some(520),
                },
                crate::models::Dish {
                    name: "清蒸鲈鱼".to_string(),
                    description: "鲜嫩多汁，保留原味".to_string(),
                    difficulty: 3,
                    time: "25分钟".to_string(),
                    calories: Some(180),
                },
                crate::models::Dish {
                    name: "宫保鸡丁".to_string(),
                    description: "酸甜微辣，经典川菜".to_string(),
                    difficulty: 3,
                    time: "30分钟".to_string(),
                    calories: Some(380),
                },
            ],
            nutrition: Some(crate::models::MenuNutrition {
                total_calories: 1080,
                protein: 65,
                carbs: 45,
                fat: 72,
            }),
        }
    }
}
