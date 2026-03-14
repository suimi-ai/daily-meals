# 实施任务清单

## 阶段 1：购物清单 API（优先级：高）

### ⬜ Task 1.1: 数据模型
- [ ] 创建 `ShoppingList` 模型
- [ ] 创建 `Ingredient` 模型
- [ ] 创建 `ShoppingSummary` 模型
- [ ] 添加 Serde 序列化

### ⬜ Task 1.2: 业务逻辑
- [ ] 实现 `ShoppingService`
- [ ] 实现食材提取逻辑
- [ ] 实现分量调整算法
- [ ] 实现分类逻辑

### ⬜ Task 1.3: API 端点
- [ ] POST `/api/shopping/generate`
- [ ] POST `/api/shopping/inventory`
- [ ] 添加参数验证
- [ ] 错误处理

## 阶段 2：菜谱查询 API（优先级：高）

### ⬜ Task 2.1: 数据模型
- [ ] 创建 `Recipe` 模型
- [ ] 创建 `RecipeStep` 模型
- [ ] 创建 `RecipeIngredient` 模型

### ⬜ Task 2.2: 业务逻辑
- [ ] 实现 `RecipeService`
- [ ] 实现菜谱查询
- [ ] 实现菜谱搜索
- [ ] 模拟数据库

### ⬜ Task 2.3: API 端点
- [ ] GET `/api/recipe/:name`
- [ ] GET `/api/recipe/search`
- [ ] 添加缓存机制
- [ ] 错误处理

## 阶段 3：API 文档（优先级：中）

### ⬜ Task 3.1: OpenAPI 规范
- [ ] 创建 `openapi.yaml`
- [ ] 定义所有数据模型
- [ ] 定义所有 API 路径
- [ ] 添加示例和描述

### ⬜ Task 3.2: Swagger UI
- [ ] 添加 `utoipa` 依赖
- [ ] 集成 Swagger UI
- [ ] 配置路由
- [ ] 测试界面

## 阶段 4：优化和完善（优先级：中）

### ⬜ Task 4.1: AI 集成
- [ ] 完善 GLM 服务
- [ ] 完善 OpenAI 服务
- [ ] 添加错误重试
- [ ] 优化降级方案

### ⬜ Task 4.2: 性能优化
- [ ] 添加缓存机制
- [ ] 连接池优化
- [ ] 异步优化
- [ ] 性能测试

### ⬜ Task 4.3: 错误处理
- [ ] 统一错误格式
- [ ] 友好错误消息
- [ ] 错误日志记录
- [ ] 错误监控

## 阶段 5：清理和文档（优先级：低）

### ⬜ Task 5.1: 移除 Node.js
- [ ] 备份 Node.js 代码（可选）
- [ ] 删除 `server/` 目录
- [ ] 更新 README
- [ ] 更新文档

### ⬜ Task 5.2: 文档完善
- [ ] 更新主 README
- [ ] 完善 Rust 指南
- [ ] 添加部署指南
- [ ] 添加性能对比

## 验收标准

### 功能完整
- [ ] 所有 API 功能可用
- [ ] 前端兼容性测试通过
- [ ] 真机测试通过

### 性能达标
- [ ] 响应时间 < 100ms
- [ ] 并发支持 > 10000 QPS
- [ ] 内存占用 < 50MB

### 文档完善
- [ ] OpenAPI 规范完整
- [ ] Swagger UI 可用
- [ ] README 清晰完整
