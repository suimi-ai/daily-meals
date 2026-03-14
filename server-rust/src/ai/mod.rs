// AI 模块 - 暂时简化实现
// 未来可以添加 GLM/OpenAI 集成

use crate::error::AppError;
use crate::models::Menu;

/// 生成菜单提示词
pub fn build_menu_prompt(
    meal_type: &str,
    servings: u8,
    preferences: &[String],
    restrictions: &[String],
) -> String {
    let mut prompt = format!("请为{}人生成一份{}菜单，", servings, meal_type);

    if !preferences.is_empty() {
        prompt.push_str(&format!("口味偏好：{}，", preferences.join("、")));
    }

    if !restrictions.is_empty() {
        prompt.push_str(&format!("饮食限制：{}，", restrictions.join("、")));
    }

    prompt.push_str(
        "\n请以JSON格式返回，包含以下字段：
- dishes: 菜品数组，每个菜品包含 name(菜名), description(描述), difficulty(难度1-5), time(预计时间)
- nutrition: 营养信息对象，包含 calories, protein, carbs, fat",
    );

    prompt
}

/// 调用 AI API（预留接口）
pub async fn call_ai_api(
    _prompt: String,
    _provider: &str,
    _api_key: &str,
) -> Result<Menu, AppError> {
    // 未来实现 AI API 调用
    // 目前返回错误，使用降级方案
    Err(AppError::AIServiceError("AI 服务暂未启用".to_string()))
}
