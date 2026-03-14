const menuService = require('../services/menuService');
const { successResponse, errorResponse } = require('../utils/response');
const { ValidationError } = require('../utils/errors');

/**
 * 生成菜单
 * POST /api/menu/generate
 * Body: { mealType, preferences, servings, restrictions }
 */
exports.generateMenu = async (req, res, next) => {
  try {
    const { mealType, preferences, servings, restrictions } = req.body;
    
    // 参数验证
    if (!mealType) {
      throw new ValidationError('请指定用餐类型（早餐/午餐/晚餐）');
    }

    const validMealTypes = ['早餐', '午餐', '晚餐'];
    if (!validMealTypes.includes(mealType)) {
      throw new ValidationError('用餐类型必须是：早餐、午餐或晚餐');
    }

    const menu = await menuService.generateMenu({
      mealType,
      preferences: preferences || [],
      servings: servings || 2,
      restrictions: restrictions || []
    });

    res.json(successResponse(menu, '菜单生成成功'));
  } catch (error) {
    next(error);
  }
};

/**
 * 获取推荐菜品
 * GET /api/menu/recommend
 */
exports.getRecommendations = async (req, res, next) => {
  try {
    const recommendations = await menuService.getRecommendations();
    res.json(successResponse(recommendations, '获取推荐成功'));
  } catch (error) {
    next(error);
  }
};
