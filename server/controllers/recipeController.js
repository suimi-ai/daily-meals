const recipeService = require('../services/recipeService');
const { successResponse } = require('../utils/response');
const { NotFoundError } = require('../utils/errors');

/**
 * 获取菜谱详情
 * GET /api/recipe/:dishName
 */
exports.getRecipe = async (req, res, next) => {
  try {
    const { dishName } = req.params;
    
    const recipe = await recipeService.getRecipe(decodeURIComponent(dishName));
    
    if (!recipe) {
      throw new NotFoundError('未找到该菜谱');
    }

    res.json(successResponse(recipe, '获取菜谱成功'));
  } catch (error) {
    next(error);
  }
};

/**
 * 搜索菜谱
 * GET /api/recipe/search?keyword=xxx
 */
exports.searchRecipes = async (req, res, next) => {
  try {
    const { keyword } = req.query;
    
    const recipes = await recipeService.searchRecipes(keyword);
    
    res.json(successResponse(recipes, '搜索菜谱成功'));
  } catch (error) {
    next(error);
  }
};
