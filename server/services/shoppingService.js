/**
 * 购物清单服务
 */
class ShoppingService {
  /**
   * 生成购物清单
   */
  async generateList({ dishes, servings, existingIngredients }) {
    // 模拟食材数据库（实际项目中应该从数据库或AI获取）
    const ingredientDatabase = {
      '红烧肉': [
        { name: '五花肉', amount: '500g', category: '肉类' },
        { name: '生姜', amount: '30g', category: '调料' },
        { name: '大葱', amount: '2根', category: '蔬菜' },
        { name: '八角', amount: '3个', category: '调料' },
        { name: '老抽', amount: '2勺', category: '调料' },
        { name: '冰糖', amount: '50g', category: '调料' }
      ],
      '清炒时蔬': [
        { name: '时令蔬菜', amount: '300g', category: '蔬菜' },
        { name: '大蒜', amount: '3瓣', category: '调料' },
        { name: '盐', amount: '适量', category: '调料' }
      ],
      '番茄蛋汤': [
        { name: '番茄', amount: '2个', category: '蔬菜' },
        { name: '鸡蛋', amount: '2个', category: '蛋类' },
        { name: '香油', amount: '几滴', category: '调料' }
      ]
    };

    // 收集所有需要的食材
    let allIngredients = [];
    dishes.forEach(dish => {
      const ingredients = ingredientDatabase[dish.name] || [];
      // 根据人数调整分量
      const adjustedIngredients = ingredients.map(ing => ({
        ...ing,
        amount: this.adjustAmount(ing.amount, servings)
      }));
      allIngredients = allIngredients.concat(adjustedIngredients);
    });

    // 合并相同食材
    const mergedIngredients = this.mergeIngredients(allIngredients);

    // 标记已有食材
    const finalList = mergedIngredients.map(item => ({
      ...item,
      status: existingIngredients.includes(item.name) ? '已有' : '需购买',
      purchaseTip: this.getPurchaseTip(item)
    }));

    // 按分类排序
    const categorizedList = this.categorizeIngredients(finalList);

    return {
      summary: {
        totalItems: finalList.length,
        needToBuy: finalList.filter(i => i.status === '需购买').length,
        alreadyHave: finalList.filter(i => i.status === '已有').length
      },
      ingredients: categorizedList
    };
  }

  /**
   * 调整食材分量
   */
  adjustAmount(amount, servings) {
    // 简单的分量调整逻辑
    const baseServings = 2;
    if (servings === baseServings) return amount;
    
    const multiplier = servings / baseServings;
    const match = amount.match(/(\d+)(.*)/);
    
    if (match) {
      const num = parseInt(match[1]);
      const unit = match[2];
      return `${Math.round(num * multiplier)}${unit}`;
    }
    
    return amount;
  }

  /**
   * 合并相同食材
   */
  mergeIngredients(ingredients) {
    const merged = {};
    
    ingredients.forEach(item => {
      if (merged[item.name]) {
        // 简单的合并逻辑（实际项目需要更复杂的单位换算）
        merged[item.name].amount += ` + ${item.amount}`;
      } else {
        merged[item.name] = { ...item };
      }
    });
    
    return Object.values(merged);
  }

  /**
   * 获取购买建议
   */
  getPurchaseTip(ingredient) {
    const tips = {
      '肉类': '建议选择新鲜、有光泽的肉品',
      '蔬菜': '选择叶片翠绿、无黄叶的新鲜蔬菜',
      '调料': '可在超市一次性购齐常用调料',
      '蛋类': '检查蛋壳完整，无裂纹'
    };
    
    return tips[ingredient.category] || '根据个人喜好选择';
  }

  /**
   * 按分类组织食材
   */
  categorizeIngredients(ingredients) {
    const categories = {};
    
    ingredients.forEach(item => {
      if (!categories[item.category]) {
        categories[item.category] = [];
      }
      categories[item.category].push(item);
    });
    
    return categories;
  }

  /**
   * 更新库存
   */
  async updateInventory(ingredients) {
    // 实际项目中应该保存到数据库
    return {
      updated: true,
      count: ingredients.length,
      timestamp: new Date().toISOString()
    };
  }
}

module.exports = new ShoppingService();
