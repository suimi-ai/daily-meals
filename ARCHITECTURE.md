# 一日三餐 - 项目架构文档

## 技术栈

### 后端
- **Node.js** + Express
- **AI集成**: 支持GLM/OpenAI/Claude
- **数据存储**: 文件系统（可扩展为数据库）

### 前端
- **微信小程序**
- **UI组件**: 自定义组件库
- **状态管理**: 小程序原生data

## 项目结构

```
一日三餐/
├── server/                    # 后端服务
│   ├── routes/               # 路由层
│   │   ├── menu.js          # 菜单相关路由
│   │   ├── shopping.js      # 购物清单路由
│   │   └── recipe.js        # 菜谱路由
│   ├── controllers/          # 控制器层
│   │   ├── menuController.js
│   │   ├── shoppingController.js
│   │   └── recipeController.js
│   ├── services/             # 业务逻辑层
│   │   ├── menuService.js   # 菜单生成逻辑
│   │   ├── shoppingService.js # 购物清单逻辑
│   │   └── recipeService.js  # 菜谱查询逻辑
│   ├── config/               # 配置文件
│   ├── utils/                # 工具函数
│   ├── index.js              # 入口文件
│   └── package.json
│
├── miniprogram/              # 微信小程序
│   ├── pages/               # 页面
│   │   ├── index/          # 首页
│   │   ├── menu/           # 菜单页
│   │   ├── shopping/       # 购物清单页
│   │   └── recipe/         # 菜谱页
│   ├── utils/               # 工具函数
│   │   └── api.js          # API封装
│   ├── components/          # 公共组件
│   ├── app.js
│   ├── app.json
│   └── app.wxss
│
├── docs/                     # 文档
├── data/                     # 数据存储
├── README.md
└── ARCHITECTURE.md
```

## 核心功能流程

### 1. 菜单生成流程
```
用户输入 → AI分析 → 生成菜品列表 → 用户选择 → 显示营养信息
```

### 2. 购物清单流程
```
选择菜品 → 提取食材 → 合并去重 → 标记已有 → 分类展示 → 复制/分享
```

### 3. 菜谱查询流程
```
点击菜品 → 查询菜谱 → 展示步骤 → 步骤导航 → 小贴士提示
```

## API接口设计

### 菜单相关
- `POST /api/menu/generate` - 生成菜单
- `GET /api/menu/recommend` - 获取推荐菜品

### 购物清单
- `POST /api/shopping/generate` - 生成购物清单
- `POST /api/shopping/inventory` - 更新库存

### 菜谱
- `GET /api/recipe/:dishName` - 获取菜谱详情
- `GET /api/recipe/search` - 搜索菜谱

## 数据模型

### 菜品 (Dish)
```javascript
{
  name: String,           // 菜名
  description: String,    // 描述
  difficulty: Number,     // 难度 1-5
  time: String           // 预计时间
}
```

### 食材 (Ingredient)
```javascript
{
  name: String,          // 名称
  amount: String,        // 分量
  category: String,      // 分类
  status: String,        // 状态（已有/需购买）
  purchaseTip: String    // 购买建议
}
```

### 菜谱 (Recipe)
```javascript
{
  name: String,
  description: String,
  difficulty: Number,
  time: String,
  servings: Number,
  ingredients: Array,
  steps: Array,
  nutrition: Object,
  tips: Array
}
```

## 扩展计划

### 短期
- [ ] 添加用户偏好设置
- [ ] 实现食材库存管理
- [ ] 支持分享功能

### 中期
- [ ] 集成数据库（MongoDB/PostgreSQL）
- [ ] 添加用户系统
- [ ] 支持多平台（支付宝小程序、H5）

### 长期
- [ ] AI菜谱推荐算法优化
- [ ] 社区功能（分享菜谱）
- [ ] 智能厨房设备集成
