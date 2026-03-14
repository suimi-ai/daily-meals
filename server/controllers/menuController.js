const menuService = require('../services/menuService');

/**
 * 生成菜单
 * POST /api/menu/generate
 * Body: { mealType, preferences, servings, restrictions }
 */
exports.generateMenu = async (req, res) => {
  try {
    const { mealType, preferences, servings, restrictions } = req.body;
    
    // 参数验证
    if (!mealType) {
      return res.status(400).json({ 
        success: false, 
        message: '请指定用餐类型（早餐/午餐/晚餐）' 
      });
    }

    const menu = await menuService.generateMenu({
      mealType,
      preferences: preferences || [],
      servings: servings || 2,
      restrictions: restrictions || []
    });

    res.json({
      success: true,
      data: menu
    });
  } catch (error) {
    console.error('生成菜单失败:', error);
    res.status(500).json({ 
      success: false, 
      message: '生成菜单失败，请稍后重试' 
    });
  }
};

/**
 * 获取推荐菜品
 * GET /api/menu/recommend
 */
exports.getRecommendations = async (req, res) => {
  try {
    const recommendations = await menuService.getRecommendations();
    res.json({
      success: true,
      data: recommendations
    });
  } catch (error) {
    console.error('获取推荐失败:', error);
    res.status(500).json({ 
      success: false, 
      message: '获取推荐失败' 
    });
  }
};
