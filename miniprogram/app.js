App({
  globalData: {
    userInfo: null,
    apiBaseUrl: 'http://localhost:3000/api' // 修改为实际服务器地址
  },

  onLaunch() {
    // 检查登录状态
    this.checkLoginStatus();
  },

  checkLoginStatus() {
    const userInfo = wx.getStorageSync('userInfo');
    if (userInfo) {
      this.globalData.userInfo = userInfo;
    }
  }
});
