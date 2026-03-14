# 实施任务清单

## 阶段 1：基础设施搭建

### ✅ Task 1.1: 创建 OpenAPI 目录结构
- [ ] 创建 `server/openapi/` 目录
- [ ] 创建 `server/openapi/schemas/` 目录
- [ ] 创建 `server/openapi/paths/` 目录
- [ ] 创建主规范文件 `openapi.yaml`

### ⬜ Task 1.2: 定义数据模型 Schemas
- [ ] 创建 `schemas/dish.yaml` - 菜品模型
- [ ] 创建 `schemas/ingredient.yaml` - 食材模型
- [ ] 创建 `schemas/recipe.yaml` - 菜谱模型
- [ ] 创建 `schemas/error.yaml` - 错误响应模型
- [ ] 创建 `schemas/success.yaml` - 成功响应模型

### ⬜ Task 1.3: 实现错误处理机制
- [ ] 创建 `utils/errors.js` - 自定义错误类
- [ ] 创建 `middleware/errorHandler.js` - 错误处理中间件
- [ ] 创建 `utils/response.js` - 响应格式化工具
- [ ] 更新 `index.js` 集成错误处理中间件

## 阶段 2：API 规范化

### ⬜ Task 2.1: 菜单 API 规范化
- [ ] 创建 `paths/menu.yaml` - 菜单 API 定义
- [ ] 更新 `controllers/menuController.js` - 使用统一响应格式
- [ ] 添加参数验证
- [ ] 添加错误处理

### ⬜ Task 2.2: 购物清单 API 规范化
- [ ] 创建 `paths/shopping.yaml` - 购物清单 API 定义
- [ ] 更新 `controllers/shoppingController.js` - 使用统一响应格式
- [ ] 添加参数验证
- [ ] 添加错误处理

### ⬜ Task 2.3: 菜谱 API 规范化
- [ ] 创建 `paths/recipe.yaml` - 菜谱 API 定义
- [ ] 更新 `controllers/recipeController.js` - 使用统一响应格式
- [ ] 添加参数验证
- [ ] 添加错误处理

## 阶段 3：文档和测试

### ⬜ Task 3.1: 集成 Swagger UI
- [ ] 安装依赖：`swagger-ui-express`, `yamljs`
- [ ] 创建 `config/swagger.js` - Swagger 配置
- [ ] 更新 `index.js` - 挂载 Swagger UI 路由
- [ ] 测试 Swagger UI 访问

### ⬜ Task 3.2: 编写 API 文档
- [ ] 创建 `docs/api.md` - API 使用文档
- [ ] 添加快速开始指南
- [ ] 添加常见问题解答
- [ ] 添加示例代码

### ⬜ Task 3.3: 测试和验证
- [ ] 测试所有 API 接口
- [ ] 验证响应格式统一性
- [ ] 测试错误处理
- [ ] 测试 Swagger UI 功能

## 阶段 4：部署和监控

### ⬜ Task 4.1: 环境配置
- [ ] 添加 `ENABLE_SWAGGER` 环境变量
- [ ] 生产环境禁用 Swagger UI
- [ ] 更新 `.env.example`

### ⬜ Task 4.2: 提交和推送
- [ ] Git commit 所有变更
- [ ] 推送到 GitHub
- [ ] 更新 README.md

## 验收标准

- [ ] 所有 API 有完整的 OpenAPI 定义
- [ ] Swagger UI 可正常访问和使用
- [ ] 所有 API 响应格式统一
- [ ] 错误处理完善，错误信息友好
- [ ] 文档清晰完整
- [ ] 代码通过测试
