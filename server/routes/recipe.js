const express = require('express');
const router = express.Router();
const recipeController = require('../controllers/recipeController');

// 获取菜谱
router.get('/:dishName', recipeController.getRecipe);

// 搜索菜谱
router.get('/search', recipeController.searchRecipes);

module.exports = router;
