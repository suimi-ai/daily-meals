# 一日三餐 - 项目规范

## 技术栈

### 后端
- **Node.js** + Express
- **OpenAPI 3.0** - API 规范
- **Swagger UI** - API 文档和测试
- **AI API** - GLM / OpenAI / Claude

### 前端
- **微信小程序** - 原生开发

## API 规范

### 响应格式

#### 成功响应
```json
{
  "success": true,
  "data": {...},
  "message": "操作成功"
}
```

#### 错误响应
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "错误描述",
    "timestamp": "2024-03-14T12:00:00.000Z"
  }
}
```

### 错误码
- `VALIDATION_ERROR` - 参数验证失败 (400)
- `NOT_FOUND` - 资源不存在 (404)
- `AI_SERVICE_ERROR` - AI 服务错误 (500)
- `INTERNAL_ERROR` - 内部错误 (500)

## 开发流程

### OpenSpec 工作流
1. **Propose** - 创建规范提案
2. **Apply** - 实施开发
3. **Archive** - 归档总结

### 代码规范
- 使用统一响应格式
- 使用自定义错误类
- 编写清晰的注释
- 遵循 RESTful API 设计

## 部署配置

### 环境变量
```bash
NODE_ENV=production
ENABLE_SWAGGER=false
AI_PROVIDER=glm
GLM_API_KEY=your_key
```

## 已完成功能

### v1.0 (2026-03-14)
- ✅ 菜单生成 API
- ✅ 购物清单 API
- ✅ 菜谱查询 API
- ✅ OpenAPI 规范
- ✅ Swagger UI
- ✅ 统一错误处理
- ✅ 微信小程序前端

## 文档资源
- [OpenAPI 规范](../server/openapi/openapi.yaml)
- [API 使用文档](../server/docs/api.md)
- [后端 README](../server/README.md)
- [项目架构](../ARCHITECTURE.md)
