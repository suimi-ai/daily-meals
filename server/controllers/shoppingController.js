const shoppingService = require('../services/shoppingService');

/**
 * 生成购物清单
 * POST /api/shopping/generate
 * Body: { dishes, servings, existingIngredients }
 */
exports.generateList = async (req, res) => {
  try {
    const { dishes, servings, existingIngredients } = req.body;
    
    if (!dishes || dishes.length === 0) {
      return res.status(400).json({ 
        success: false, 
        message: '请选择要制作的菜品' 
      });
    }

    const shoppingList = await shoppingService.generateList({
      dishes,
      servings: servings || 2,
      existingIngredients: existingIngredients || []
    });

    res.json({
      success: true,
      data: shoppingList
    });
  } catch (error) {
    console.error('生成购物清单失败:', error);
    res.status(500).json({ 
      success: false, 
      message: '生成购物清单失败' 
    });
  }
};

/**
 * 更新现有食材库存
 * POST /api/shopping/inventory
 */
exports.updateInventory = async (req, res) => {
  try {
    const { ingredients } = req.body;
    
    const inventory = await shoppingService.updateInventory(ingredients);
    
    res.json({
      success: true,
      data: inventory
    });
  } catch (error) {
    console.error('更新库存失败:', error);
    res.status(500).json({ 
      success: false, 
      message: '更新库存失败' 
    });
  }
};
