# 一日三餐 - 后端服务

基于 Node.js + Express 的智能餐饮规划 API 服务

## 功能特性

- 🍽️ **智能菜单生成** - AI 驱动的个性化菜单推荐
- 🛒 **购物清单管理** - 自动生成食材清单
- 👨‍🍳 **烹饪指导** - 详细的制作步骤和技巧
- 📚 **Swagger 文档** - 交互式 API 文档和测试界面
- ⚡ **统一响应格式** - 标准化的 API 响应
- 🛡️ **错误处理** - 完善的错误处理机制

## 技术栈

- **Node.js** - 运行环境
- **Express** - Web 框架
- **OpenAPI/Swagger** - API 规范和文档
- **AI API** - GLM / OpenAI / Claude

## 快速开始

### 1. 安装依赖
```bash
npm install
```

### 2. 配置环境变量
```bash
cp .env.example .env
# 编辑 .env 文件，填入 AI API 密钥
```

### 3. 启动服务
```bash
# 开发模式
npm run dev

# 生产模式
npm start
```

### 4. 访问文档
- API 文档: http://localhost:3000/api-docs
- 健康检查: http://localhost:3000/health

## 项目结构

```
server/
├── openapi/              # OpenAPI 规范
│   ├── openapi.yaml     # 主规范文件
│   ├── schemas/         # 数据模型
│   └── paths/           # API 路径定义
├── routes/              # 路由层
├── controllers/         # 控制器层
├── services/            # 业务逻辑层
├── middleware/          # 中间件
│   └── errorHandler.js  # 错误处理
├── utils/               # 工具函数
│   ├── errors.js       # 错误类定义
│   └── response.js     # 响应格式化
├── config/              # 配置文件
│   └── swagger.js      # Swagger 配置
├── docs/                # 文档
├── index.js             # 入口文件
└── package.json
```

## API 端点

### 菜单生成
- `POST /api/menu/generate` - 生成菜单
- `GET /api/menu/recommend` - 获取推荐

### 购物清单
- `POST /api/shopping/generate` - 生成购物清单
- `POST /api/shopping/inventory` - 更新库存

### 菜谱查询
- `GET /api/recipe/:dishName` - 获取菜谱详情
- `GET /api/recipe/search` - 搜索菜谱

## 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| PORT | 服务端口 | 3000 |
| NODE_ENV | 运行环境 | development |
| AI_PROVIDER | AI 提供商 | glm |
| GLM_API_KEY | GLM API 密钥 | - |
| OPENAI_API_KEY | OpenAI API 密钥 | - |
| CLAUDE_API_KEY | Claude API 密钥 | - |
| ENABLE_SWAGGER | 启用 Swagger UI | true |

## 开发指南

### 添加新 API

1. 在 `openapi/paths/` 添加 API 定义
2. 在 `routes/` 添加路由
3. 在 `controllers/` 添加控制器
4. 在 `services/` 实现业务逻辑
5. 使用统一响应格式

### 错误处理

使用自定义错误类：
```javascript
const { ValidationError, NotFoundError } = require('./utils/errors');

// 在控制器中
if (!param) {
  throw new ValidationError('参数不能为空');
}
```

### 响应格式

使用统一响应工具：
```javascript
const { successResponse, errorResponse } = require('./utils/response');

// 成功响应
res.json(successResponse(data, '操作成功'));

// 错误响应（由错误处理中间件自动处理）
```

## 测试

```bash
# 使用 Swagger UI 测试
访问 http://localhost:3000/api-docs

# 或使用 curl
curl -X POST http://localhost:3000/api/menu/generate \
  -H "Content-Type: application/json" \
  -d '{"mealType":"午餐","servings":2}'
```

## 部署

### 生产环境配置
```bash
NODE_ENV=production
ENABLE_SWAGGER=false
```

### 使用 PM2
```bash
pm2 start index.js --name daily-meals-api
```

## 许可证

MIT
