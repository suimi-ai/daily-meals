# 技术设计

## 架构设计

### 1. 目录结构
```
server/
├── openapi/
│   ├── openapi.yaml          # 主规范文件
│   ├── schemas/              # 数据模型定义
│   │   ├── dish.yaml
│   │   ├── ingredient.yaml
│   │   └── recipe.yaml
│   └── paths/                # API 路径定义
│       ├── menu.yaml
│       ├── shopping.yaml
│       └── recipe.yaml
├── middleware/
│   └── errorHandler.js       # 统一错误处理
└── docs/
    └── api.md               # API 文档
```

### 2. 技术选型

#### OpenAPI 工具
- **swagger-ui-express** - 提供 Swagger UI 界面
- **yamljs** - 解析 YAML 格式的 OpenAPI 文件
- **ajv** - JSON Schema 验证

#### 错误处理
- 自定义错误类 `AppError`
- 错误处理中间件
- 错误日志记录

## 详细设计

### 1. OpenAPI 规范结构

#### 主文件 (openapi.yaml)
```yaml
openapi: 3.0.0
info:
  title: 一日三餐 API
  version: 1.0.0
  description: AI驱动的智能餐饮规划助手 API

servers:
  - url: http://localhost:3000/api
    description: 开发环境

paths:
  /menu/generate:
    $ref: './paths/menu.yaml#/generate'
  
  /shopping/generate:
    $ref: './paths/shopping.yaml#/generate'
  
  /recipe/{dishName}:
    $ref: './paths/recipe.yaml#/get'
```

#### Schema 示例 (schemas/dish.yaml)
```yaml
Dish:
  type: object
  required:
    - name
    - description
    - difficulty
    - time
  properties:
    name:
      type: string
      example: 红烧肉
    description:
      type: string
      example: 肥而不腻，入口即化
    difficulty:
      type: integer
      minimum: 1
      maximum: 5
      example: 4
    time:
      type: string
      example: 90分钟
```

### 2. 统一响应格式

#### 成功响应包装器
```javascript
function successResponse(data, message = 'Success') {
  return {
    success: true,
    data,
    message
  };
}
```

#### 错误响应包装器
```javascript
function errorResponse(code, message, details = null) {
  return {
    success: false,
    error: {
      code,
      message,
      details,
      timestamp: new Date().toISOString()
    }
  };
}
```

### 3. 错误处理中间件

```javascript
// 自定义错误类
class AppError extends Error {
  constructor(code, message, statusCode = 500) {
    super(message);
    this.code = code;
    this.statusCode = statusCode;
  }
}

// 错误处理中间件
function errorHandler(err, req, res, next) {
  if (err instanceof AppError) {
    return res.status(err.statusCode).json(
      errorResponse(err.code, err.message)
    );
  }
  
  // 未知错误
  console.error(err);
  res.status(500).json(
    errorResponse('INTERNAL_ERROR', '服务器内部错误')
  );
}
```

### 4. Swagger UI 集成

```javascript
const swaggerUi = require('swagger-ui-express');
const YAML = require('yamljs');
const swaggerDocument = YAML.load('./openapi/openapi.yaml');

// 仅在开发环境启用
if (process.env.NODE_ENV !== 'production') {
  app.use('/api-docs', swaggerUi.serve, swaggerUi.setup(swaggerDocument));
}
```

## 实施策略

### 阶段 1：基础设施（第1天）
- 创建 OpenAPI 目录结构
- 定义基础 schemas
- 实现错误处理中间件

### 阶段 2：API 规范化（第2天）
- 为现有 API 添加 OpenAPI 定义
- 统一响应格式
- 添加参数验证

### 阶段 3：文档和测试（第3天）
- 集成 Swagger UI
- 编写 API 文档
- 添加示例和测试用例

## 风险和缓解

### 风险 1：响应格式变更影响前端
**缓解**: 保持向后兼容，旧格式继续支持 1 个月

### 风险 2：OpenAPI 文档维护成本
**缓解**: 使用工具自动生成部分文档，定期 review

## 性能影响

- OpenAPI 规范文件加载：~50ms（启动时）
- Swagger UI：仅开发环境，无生产影响
- 响应包装：~1ms（可忽略）
