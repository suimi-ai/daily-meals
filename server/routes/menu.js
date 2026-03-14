const express = require('express');
const router = express.Router();
const menuController = require('../controllers/menuController');

// 生成菜单
router.post('/generate', menuController.generateMenu);

// 获取推荐菜品
router.get('/recommend', menuController.getRecommendations);

module.exports = router;
