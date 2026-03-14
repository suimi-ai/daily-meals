const api = require('../../utils/api');

Page({
  data: {
    shoppingList: null,
    loading: true
  },

  onLoad(options) {
    if (options.dishes) {
      const dishes = JSON.parse(decodeURIComponent(options.dishes));
      this.generateShoppingList(dishes);
    }
  },

  // 生成购物清单
  async generateShoppingList(dishes) {
    try {
      const list = await api.post('/shopping/generate', {
        dishes: dishes,
        servings: 2,
        existingIngredients: []
      });

      this.setData({
        shoppingList: list,
        loading: false
      });
    } catch (error) {
      console.error('生成购物清单失败:', error);
      this.setData({ loading: false });
    }
  },

  // 复制清单
  copyList() {
    let text = '🛒 购物清单\n\n';
    
    for (const [category, items] of Object.entries(this.data.shoppingList.ingredients)) {
      text += `【${category}】\n`;
      items.forEach(item => {
        text += `${item.name} ${item.amount} - ${item.status}\n`;
      });
      text += '\n';
    }

    wx.setClipboardData({
      data: text,
      success() {
        wx.showToast({
          title: '已复制到剪贴板',
          icon: 'success'
        });
      }
    });
  },

  // 标记已购买
  markPurchased(e) {
    const { category, index } = e.currentTarget.dataset;
    const key = `shoppingList.ingredients.${category}[${index}].purchased`;
    const purchased = !this.data.shoppingList.ingredients[category][index].purchased;
    
    this.setData({
      [key]: purchased
    });
  }
});
