# API 使用文档

## 快速开始

### 1. 启动服务
```bash
cd server
npm install
npm run dev
```

### 2. 访问 API 文档
- **Swagger UI**: http://localhost:3000/api-docs
- **健康检查**: http://localhost:3000/health
- **API 信息**: http://localhost:3000/

## API 端点

### 菜单生成

#### 生成菜单
```http
POST /api/menu/generate
Content-Type: application/json

{
  "mealType": "午餐",
  "servings": 2,
  "preferences": [],
  "restrictions": []
}
```

**响应示例**:
```json
{
  "success": true,
  "data": {
    "dishes": [
      {
        "name": "红烧肉",
        "description": "肥而不腻，入口即化",
        "difficulty": 4,
        "time": "90分钟"
      }
    ],
    "nutrition": {
      "calories": 650,
      "protein": 25,
      "carbs": 45,
      "fat": 35
    }
  },
  "message": "菜单生成成功"
}
```

#### 获取推荐菜品
```http
GET /api/menu/recommend
```

### 购物清单

#### 生成购物清单
```http
POST /api/shopping/generate
Content-Type: application/json

{
  "dishes": [
    {"name": "红烧肉"},
    {"name": "清炒时蔬"}
  ],
  "servings": 2,
  "existingIngredients": []
}
```

#### 更新库存
```http
POST /api/shopping/inventory
Content-Type: application/json

{
  "ingredients": ["盐", "酱油"]
}
```

### 菜谱查询

#### 获取菜谱详情
```http
GET /api/recipe/红烧肉
```

#### 搜索菜谱
```http
GET /api/recipe/search?keyword=红烧
```

## 错误处理

### 错误响应格式
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "请指定用餐类型（早餐/午餐/晚餐）",
    "timestamp": "2024-03-14T12:00:00.000Z"
  }
}
```

### 错误码说明

| 错误码 | 说明 | HTTP 状态码 |
|--------|------|------------|
| VALIDATION_ERROR | 参数验证失败 | 400 |
| NOT_FOUND | 资源不存在 | 404 |
| AI_SERVICE_ERROR | AI 服务调用失败 | 500 |
| INTERNAL_ERROR | 内部错误 | 500 |
| INVALID_JSON | JSON 格式错误 | 400 |

## 小程序集成

### 1. 修改 API 地址
编辑 `miniprogram/utils/api.js`:
```javascript
apiBaseUrl: 'http://your-server:3000/api'
```

### 2. 测试 API
使用 Swagger UI 测试所有接口，确保响应格式正确。

## 开发建议

1. **先测试后开发**: 使用 Swagger UI 测试 API
2. **统一错误处理**: 所有错误都会返回统一格式
3. **查看日志**: 错误信息会记录在服务器日志中
4. **环境变量**: 生产环境记得设置 `ENABLE_SWAGGER=false`

## 常见问题

### Q: Swagger UI 无法访问？
A: 检查 `ENABLE_SWAGGER` 环境变量是否为 `true`

### Q: API 返回 500 错误？
A: 查看服务器日志，检查 AI API 配置

### Q: 小程序无法调用 API？
A: 检查网络请求配置，确保域名白名单设置正确

---

更多信息请参考 [Swagger UI](http://localhost:3000/api-docs)
