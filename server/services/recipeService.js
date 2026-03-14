/**
 * 菜谱服务
 */
class RecipeService {
  constructor() {
    // 模拟菜谱数据库
    this.recipeDatabase = {
      '红烧肉': {
        name: '红烧肉',
        description: '经典家常菜，肥而不腻，入口即化',
        difficulty: 4,
        time: '90分钟',
        servings: 4,
        ingredients: [
          { name: '五花肉', amount: '500g' },
          { name: '生姜', amount: '30g' },
          { name: '大葱', amount: '2根' },
          { name: '八角', amount: '3个' },
          { name: '老抽', amount: '2勺' },
          { name: '生抽', amount: '3勺' },
          { name: '冰糖', amount: '50g' },
          { name: '料酒', amount: '2勺' }
        ],
        steps: [
          {
            step: 1,
            title: '准备食材',
            description: '五花肉切成3-4cm见方的块，生姜切片，大葱切段',
            time: '10分钟',
            tips: '选择肥瘦相间的五花肉，口感最佳'
          },
          {
            step: 2,
            title: '焯水去腥',
            description: '肉块冷水下锅，加入料酒和姜片，煮沸后撇去浮沫，捞出沥干',
            time: '10分钟',
            tips: '焯水可以去除血水和腥味'
          },
          {
            step: 3,
            title: '炒糖色',
            description: '锅中放少许油，加入冰糖，小火炒至焦糖色',
            time: '5分钟',
            tips: '糖色不能炒过，否则会发苦'
          },
          {
            step: 4,
            title: '上色翻炒',
            description: '放入肉块翻炒上色，加入老抽、生抽、料酒',
            time: '5分钟',
            tips: '翻炒均匀，让每块肉都上色'
          },
          {
            step: 5,
            title: '焖煮',
            description: '加入开水没过肉块，放入八角、姜片、葱段，大火烧开后转小火焖煮60分钟',
            time: '60分钟',
            tips: '水要一次加够，中途不要开盖'
          },
          {
            step: 6,
            title: '收汁',
            description: '最后大火收汁，汤汁浓稠即可出锅',
            time: '5分钟',
            tips: '收汁时要不断翻动，避免粘锅'
          }
        ],
        nutrition: {
          calories: 520,
          protein: 25,
          carbs: 15,
          fat: 42
        },
        tips: [
          '选用五花肉要肥瘦相间，这样烧出来才好吃',
          '炒糖色是关键，决定了红烧肉的颜色和口感',
          '焖煮时间要够长，才能达到入口即化的效果'
        ]
      }
    };
  }

  /**
   * 获取菜谱
   */
  async getRecipe(dishName) {
    // 先从数据库查找
    let recipe = this.recipeDatabase[dishName];
    
    if (!recipe) {
      // 如果没有，尝试用AI生成
      recipe = await this.generateRecipeByAI(dishName);
    }
    
    return recipe;
  }

  /**
   * 使用AI生成菜谱
   */
  async generateRecipeByAI(dishName) {
    // 实际项目中应该调用AI API
    // 这里返回模拟数据
    return {
      name: dishName,
      description: `AI生成的${dishName}菜谱`,
      difficulty: 3,
      time: '30分钟',
      servings: 2,
      ingredients: [
        { name: '主料', amount: '适量' },
        { name: '调料', amount: '适量' }
      ],
      steps: [
        {
          step: 1,
          title: '准备',
          description: '准备好所有食材',
          time: '5分钟',
          tips: ''
        },
        {
          step: 2,
          title: '烹饪',
          description: '按照常规方法烹饪',
          time: '20分钟',
          tips: ''
        },
        {
          step: 3,
          title: '完成',
          description: '装盘即可享用',
          time: '5分钟',
          tips: ''
        }
      ],
      nutrition: {
        calories: 300,
        protein: 15,
        carbs: 20,
        fat: 18
      },
      tips: ['这是一道美味佳肴']
    };
  }

  /**
   * 搜索菜谱
   */
  async searchRecipes(keyword) {
    const results = [];
    
    for (const [name, recipe] of Object.entries(this.recipeDatabase)) {
      if (name.includes(keyword) || recipe.description.includes(keyword)) {
        results.push({
          name: recipe.name,
          description: recipe.description,
          difficulty: recipe.difficulty,
          time: recipe.time
        });
      }
    }
    
    return results;
  }
}

module.exports = new RecipeService();
