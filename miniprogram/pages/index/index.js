const api = require('../../utils/api');

Page({
  data: {
    mealTypes: ['早餐', '午餐', '晚餐'],
    selectedMeal: '午餐',
    servings: 2,
    preferences: [],
    restrictions: [],
    loading: false
  },

  onLoad() {
    // 页面加载
  },

  // 选择用餐类型
  selectMeal(e) {
    this.setData({
      selectedMeal: e.currentTarget.dataset.meal
    });
  },

  // 修改用餐人数
  changeServings(e) {
    this.setData({
      servings: parseInt(e.detail.value)
    });
  },

  // 生成菜单
  async generateMenu() {
    this.setData({ loading: true });

    try {
      const menu = await api.post('/menu/generate', {
        mealType: this.data.selectedMeal,
        servings: this.data.servings,
        preferences: this.data.preferences,
        restrictions: this.data.restrictions
      });

      // 跳转到菜单页面
      wx.navigateTo({
        url: `/pages/menu/menu?data=${encodeURIComponent(JSON.stringify(menu))}`
      });
    } catch (error) {
      console.error('生成菜单失败:', error);
    } finally {
      this.setData({ loading: false });
    }
  },

  // 查看推荐
  viewRecommendations() {
    wx.navigateTo({
      url: '/pages/menu/menu?type=recommend'
    });
  }
});
