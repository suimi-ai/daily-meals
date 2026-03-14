const api = require('../../utils/api');

Page({
  data: {
    menu: null,
    selectedDishes: [],
    loading: true
  },

  onLoad(options) {
    if (options.data) {
      // 从首页传来的菜单数据
      const menu = JSON.parse(decodeURIComponent(options.data));
      this.setData({
        menu: menu,
        loading: false
      });
    } else if (options.type === 'recommend') {
      // 加载推荐菜品
      this.loadRecommendations();
    }
  },

  // 加载推荐菜品
  async loadRecommendations() {
    try {
      const recommendations = await api.get('/menu/recommend');
      this.setData({
        menu: { dishes: recommendations.popular },
        loading: false
      });
    } catch (error) {
      console.error('加载推荐失败:', error);
      this.setData({ loading: false });
    }
  },

  // 选择/取消选择菜品
  toggleDish(e) {
    const dishName = e.currentTarget.dataset.dish;
    let selectedDishes = [...this.data.selectedDishes];
    
    const index = selectedDishes.findIndex(d => d.name === dishName);
    if (index > -1) {
      selectedDishes.splice(index, 1);
    } else {
      const dish = this.data.menu.dishes.find(d => d.name === dishName);
      selectedDishes.push(dish);
    }

    this.setData({ selectedDishes });
  },

  // 生成购物清单
  generateShoppingList() {
    if (this.data.selectedDishes.length === 0) {
      wx.showToast({
        title: '请先选择菜品',
        icon: 'none'
      });
      return;
    }

    const dishesData = encodeURIComponent(JSON.stringify(this.data.selectedDishes));
    wx.navigateTo({
      url: `/pages/shopping/shopping?dishes=${dishesData}`
    });
  },

  // 查看菜谱
  viewRecipe(e) {
    const dishName = encodeURIComponent(e.currentTarget.dataset.dish);
    wx.navigateTo({
      url: `/pages/recipe/recipe?name=${dishName}`
    });
  }
});
