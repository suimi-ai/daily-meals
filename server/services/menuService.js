const axios = require('axios');

/**
 * 生成菜单服务
 */
class MenuService {
  constructor() {
    this.apiProvider = process.env.AI_PROVIDER || 'glm';
  }

  /**
   * 调用AI API生成菜单
   */
  async generateMenu({ mealType, preferences, servings, restrictions }) {
    const prompt = this.buildPrompt(mealType, preferences, servings, restrictions);
    
    try {
      // 根据配置选择AI提供商
      let menu;
      switch (this.apiProvider) {
        case 'glm':
          menu = await this.callGLMAPI(prompt);
          break;
        case 'openai':
          menu = await this.callOpenAI(prompt);
          break;
        case 'claude':
          menu = await this.callClaude(prompt);
          break;
        default:
          menu = await this.callGLMAPI(prompt);
      }

      return menu;
    } catch (error) {
      console.error('AI API调用失败:', error);
      // 返回模拟数据作为降级方案
      return this.getMockMenu(mealType);
    }
  }

  /**
   * 构建提示词
   */
  buildPrompt(mealType, preferences, servings, restrictions) {
    let prompt = `请为${servings}人生成一份${mealType}菜单，`;
    
    if (preferences.length > 0) {
      prompt += `口味偏好：${preferences.join('、')}，`;
    }
    
    if (restrictions.length > 0) {
      prompt += `饮食限制：${restrictions.join('、')}，`;
    }
    
    prompt += `\n请以JSON格式返回，包含以下字段：
- dishes: 菜品数组，每个菜品包含 name(菜名), description(描述), difficulty(难度1-5), time(预计时间)
- nutrition: 营养信息对象，包含 calories, protein, carbs, fat`;

    return prompt;
  }

  /**
   * 调用GLM API
   */
  async callGLMAPI(prompt) {
    const response = await axios.post('https://open.bigmodel.cn/api/paas/v3/model-api/chatglm_pro/invoke', {
      prompt: prompt,
      temperature: 0.7,
      top_p: 0.9
    }, {
      headers: {
        'Authorization': `Bearer ${process.env.GLM_API_KEY}`,
        'Content-Type': 'application/json'
      }
    });

    return this.parseAIResponse(response.data.data.choices[0].content);
  }

  /**
   * 调用OpenAI API
   */
  async callOpenAI(prompt) {
    const response = await axios.post('https://api.openai.com/v1/chat/completions', {
      model: 'gpt-3.5-turbo',
      messages: [{ role: 'user', content: prompt }],
      temperature: 0.7
    }, {
      headers: {
        'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`,
        'Content-Type': 'application/json'
      }
    });

    return this.parseAIResponse(response.data.choices[0].message.content);
  }

  /**
   * 调用Claude API
   */
  async callClaude(prompt) {
    const response = await axios.post('https://api.anthropic.com/v1/messages', {
      model: 'claude-3-sonnet-20240229',
      max_tokens: 1024,
      messages: [{ role: 'user', content: prompt }]
    }, {
      headers: {
        'x-api-key': process.env.CLAUDE_API_KEY,
        'anthropic-version': '2023-06-01',
        'Content-Type': 'application/json'
      }
    });

    return this.parseAIResponse(response.data.content[0].text);
  }

  /**
   * 解析AI响应
   */
  parseAIResponse(content) {
    try {
      // 尝试提取JSON
      const jsonMatch = content.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        return JSON.parse(jsonMatch[0]);
      }
    } catch (error) {
      console.error('解析AI响应失败:', error);
    }
    
    // 如果解析失败，返回模拟数据
    return this.getMockMenu('午餐');
  }

  /**
   * 模拟数据（降级方案）
   */
  getMockMenu(mealType) {
    const menus = {
      '早餐': {
        dishes: [
          { name: '鸡蛋灌饼', description: '酥脆的饼皮包裹嫩滑鸡蛋', difficulty: 2, time: '15分钟' },
          { name: '小米粥', description: '养胃暖身，搭配小菜更美味', difficulty: 1, time: '30分钟' },
          { name: '凉拌黄瓜', description: '清爽开胃，简单快手', difficulty: 1, time: '5分钟' }
        ],
        nutrition: { calories: 450, protein: 15, carbs: 60, fat: 18 }
      },
      '午餐': {
        dishes: [
          { name: '红烧肉', description: '肥而不腻，入口即化', difficulty: 4, time: '90分钟' },
          { name: '清炒时蔬', description: '新鲜蔬菜，清淡爽口', difficulty: 2, time: '10分钟' },
          { name: '番茄蛋汤', description: '酸甜开胃，营养丰富', difficulty: 2, time: '15分钟' }
        ],
        nutrition: { calories: 650, protein: 25, carbs: 45, fat: 35 }
      },
      '晚餐': {
        dishes: [
          { name: '清蒸鲈鱼', description: '鲜嫩多汁，保留原味', difficulty: 3, time: '25分钟' },
          { name: '蒜蓉西兰花', description: '营养健康，色彩诱人', difficulty: 2, time: '10分钟' },
          { name: '紫菜蛋花汤', description: '清淡鲜美，助消化', difficulty: 1, time: '10分钟' }
        ],
        nutrition: { calories: 400, protein: 30, carbs: 25, fat: 15 }
      }
    };

    return menus[mealType] || menus['午餐'];
  }

  /**
   * 获取推荐菜品
   */
  async getRecommendations() {
    return {
      popular: [
        { name: '宫保鸡丁', rating: 4.8, orders: 1256 },
        { name: '麻婆豆腐', rating: 4.7, orders: 987 },
        { name: '糖醋里脊', rating: 4.9, orders: 1532 }
      ],
      seasonal: [
        { name: '春笋炒肉', reason: '春季时令' },
        { name: '韭菜盒子', reason: '春季养生' }
      ]
    };
  }
}

module.exports = new MenuService();
