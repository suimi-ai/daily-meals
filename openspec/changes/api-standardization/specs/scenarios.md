# 场景说明

## 场景 1：开发者查看 API 文档

**角色**: 前端开发者

**前置条件**:
- 后端服务已启动
- 访问 http://localhost:3000/api-docs

**流程**:
1. 打开 Swagger UI 界面
2. 浏览可用的 API 列表
3. 查看某个 API 的详细说明
4. 点击 "Try it out" 测试 API
5. 查看响应结果

**预期结果**:
- 清晰的 API 文档展示
- 可以直接测试 API
- 响应格式符合规范

---

## 场景 2：API 调用成功

**角色**: 小程序前端

**前置条件**:
- 用户选择菜品生成菜单

**流程**:
1. 小程序发送 POST /api/menu/generate
2. 后端处理请求
3. 返回统一格式的成功响应

**请求示例**:
```json
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
    "dishes": [...],
    "nutrition": {...}
  }
}
```

---

## 场景 3：API 调用失败

**角色**: 小程序前端

**前置条件**:
- 用户未提供必需参数

**流程**:
1. 小程序发送 POST /api/menu/generate（缺少 mealType）
2. 后端验证参数失败
3. 返回统一格式的错误响应

**响应示例**:
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "请指定用餐类型（早餐/午餐/晚餐）"
  }
}
```

---

## 场景 4：错误码处理

**错误码定义**:

| 错误码 | 说明 | HTTP 状态码 |
|--------|------|------------|
| VALIDATION_ERROR | 参数验证失败 | 400 |
| NOT_FOUND | 资源不存在 | 404 |
| AI_SERVICE_ERROR | AI 服务调用失败 | 500 |
| INTERNAL_ERROR | 内部错误 | 500 |
