use crate::error::AppError;
use crate::models::Menu;

/// 构建菜单生成提示词
#[allow(dead_code)]
pub fn build_menu_prompt(
    meal_type: &str,
    servings: u8,
    preferences: &[String],
    restrictions: &[String],
) -> String {
    format!(
        "请为{}生成{}人份的菜单。
饮食偏好：{}
饮食限制：{}
请返回 JSON 格式的菜单，包含菜品名称、描述、难度、时间等信息。",
        meal_type,
        servings,
        preferences.join("、"),
        restrictions.join("、")
    )
}

/// 调用 AI API 生成菜单
#[allow(dead_code)]
pub async fn call_ai_api(
    _prompt: String,
    _provider: &str,
    _api_key: &str,
) -> Result<Menu, AppError> {
    // TODO: 实现真实的 AI API 调用
    // 目前返回模拟数据
    Err(AppError::InternalError(
        "AI service not implemented".to_string(),
    ))
}
