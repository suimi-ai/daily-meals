# 提案：API 规范化与优化

## 为什么做这个变更？

当前项目已经实现了基础功能，但 API 缺乏规范化文档和标准定义。需要：
1. 添加 OpenAPI Specification 规范
2. 统一 API 响应格式
3. 添加错误处理机制
4. 提供 API 文档和测试界面

## 将改变什么？

### 新增
- `server/openapi.yaml` - OpenAPI 规范文件
- `server/docs/api.md` - API 文档
- Swagger UI 界面 - 交互式 API 测试

### 修改
- 统一所有 API 响应格式
- 添加错误码定义
- 优化错误处理中间件

### 移除
- 临时的模拟数据逻辑（保留作为降级方案）

## 影响范围
- 后端服务 (server/)
- API 文档 (docs/)
- 前端调用方式（保持向后兼容）
