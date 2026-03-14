const shoppingService = require('../services/shoppingService');
const { successResponse } = require('../utils/response');
const { ValidationError } = require('../utils/errors');

/**
 * 生成购物清单
 * POST /api/shopping/generate
 * Body: { dishes, servings, existingIngredients }
 */
exports.generateList = async (req, res, next) => {
  try {
    const { dishes, servings, existingIngredients } = req.body;
    
    if (!dishes || !Array.isArray(dishes) || dishes.length === 0) {
      throw new ValidationError('请选择要制作的菜品');
    }

    const shoppingList = await shoppingService.generateList({
      dishes,
      servings: servings || 2,
      existingIngredients: existingIngredients || []
    });

    res.json(successResponse(shoppingList, '购物清单生成成功'));
  } catch (error) {
    next(error);
  }
};

/**
 * 更新现有食材库存
 * POST /api/shopping/inventory
 */
exports.updateInventory = async (req, res, next) => {
  try {
    const { ingredients } = req.body;
    
    const inventory = await shoppingService.updateInventory(ingredients);
    
    res.json(successResponse(inventory, '库存更新成功'));
  } catch (error) {
    next(error);
  }
};
