const app = getApp();

/**
 * 封装请求方法
 */
function request(url, method, data) {
  return new Promise((resolve, reject) => {
    wx.showLoading({
      title: '加载中...',
      mask: true
    });

    wx.request({
      url: `${app.globalData.apiBaseUrl}${url}`,
      method: method,
      data: data,
      header: {
        'content-type': 'application/json'
      },
      success(res) {
        wx.hideLoading();
        
        if (res.data.success) {
          resolve(res.data.data);
        } else {
          wx.showToast({
            title: res.data.message || '请求失败',
            icon: 'none',
            duration: 2000
          });
          reject(res.data);
        }
      },
      fail(error) {
        wx.hideLoading();
        wx.showToast({
          title: '网络请求失败',
          icon: 'none',
          duration: 2000
        });
        reject(error);
      }
    });
  });
}

/**
 * GET请求
 */
function get(url, data) {
  return request(url, 'GET', data);
}

/**
 * POST请求
 */
function post(url, data) {
  return request(url, 'POST', data);
}

module.exports = {
  get,
  post
};
