const recipeService = require('../services/recipeService');

/**
 * 获取菜谱详情
 * GET /api/recipe/:dishName
 */
exports.getRecipe = async (req, res) => {
  try {
    const { dishName } = req.params;
    
    const recipe = await recipeService.getRecipe(decodeURIComponent(dishName));
    
    if (!recipe) {
      return res.status(404).json({ 
        success: false, 
        message: '未找到该菜谱' 
      });
    }

    res.json({
      success: true,
      data: recipe
    });
  } catch (error) {
    console.error('获取菜谱失败:', error);
    res.status(500).json({ 
      success: false, 
      message: '获取菜谱失败' 
    });
  }
};

/**
 * 搜索菜谱
 * GET /api/recipe/search?keyword=xxx
 */
exports.searchRecipes = async (req, res) => {
  try {
    const { keyword } = req.query;
    
    const recipes = await recipeService.searchRecipes(keyword);
    
    res.json({
      success: true,
      data: recipes
    });
  } catch (error) {
    console.error('搜索菜谱失败:', error);
    res.status(500).json({ 
      success: false, 
      message: '搜索菜谱失败' 
    });
  }
};
