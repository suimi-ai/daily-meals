const express = require('express');
const router = express.Router();
const shoppingController = require('../controllers/shoppingController');

// 生成购物清单
router.post('/generate', shoppingController.generateList);

// 更新现有食材
router.post('/inventory', shoppingController.updateInventory);

module.exports = router;
