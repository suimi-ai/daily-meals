const api = require('../../utils/api');

Page({
  data: {
    recipe: null,
    loading: true,
    currentStep: 0
  },

  onLoad(options) {
    if (options.name) {
      const dishName = decodeURIComponent(options.name);
      this.loadRecipe(dishName);
    }
  },

  // 加载菜谱
  async loadRecipe(dishName) {
    try {
      const recipe = await api.get(`/recipe/${encodeURIComponent(dishName)}`);
      this.setData({
        recipe: recipe,
        loading: false
      });

      // 设置页面标题
      wx.setNavigationBarTitle({
        title: recipe.name
      });
    } catch (error) {
      console.error('加载菜谱失败:', error);
      this.setData({ loading: false });
    }
  },

  // 上一步
  prevStep() {
    if (this.data.currentStep > 0) {
      this.setData({
        currentStep: this.data.currentStep - 1
      });
    }
  },

  // 下一步
  nextStep() {
    if (this.data.currentStep < this.data.recipe.steps.length - 1) {
      this.setData({
        currentStep: this.data.currentStep + 1
      });
    }
  },

  // 切换到指定步骤
  goToStep(e) {
    const step = e.currentTarget.dataset.step;
    this.setData({
      currentStep: step
    });
  }
});
