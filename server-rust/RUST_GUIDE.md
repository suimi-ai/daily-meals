# 一日三餐 Rust 后端 - 快速开始

## ✅ Rust 已安装

```
rustc 1.94.0
cargo 1.94.0
```

---

## 🚀 快速开始

### 1. 配置环境变量
```bash
cd ~/一日三餐/server-rust
cp .env.example .env
# 编辑 .env 文件，填入你的 AI API 密钥
```

### 2. 构建项目
```bash
cargo build
```

### 3. 运行服务
```bash
cargo run
```

### 4. 测试 API
```bash
# 健康检查
curl http://localhost:3000/health

# 生成菜单
curl -X POST http://localhost:3000/api/menu/generate \
  -H "Content-Type: application/json" \
  -d '{"mealType":"午餐","servings":2,"preferences":[],"restrictions":[]}'
```

---

## 📊 性能测试

### 安装 wrk（HTTP 压测工具）
```bash
brew install wrk  # macOS
```

### 性能测试
```bash
# 测试健康检查接口
wrk -t4 -c1000 -d30s http://localhost:3000/health

# 预期结果：
# - QPS: 50000+
# - 延迟: <10ms
# - 内存: <30MB
```

---

## 🔧 开发模式

### 热重载（使用 cargo-watch）
```bash
cargo install cargo-watch
cargo watch -x run
```

### 代码格式化
```bash
cargo fmt
```

### 代码检查
```bash
cargo clippy
```

---

## 📦 生产构建

### 优化编译
```bash
cargo build --release
```

### 运行生产版本
```bash
./target/release/daily-meals
```

### 性能对比
| 版本 | 文件大小 | 启动时间 | 内存占用 |
|------|---------|---------|---------|
| Debug | ~50MB | ~2s | ~50MB |
| Release | ~10MB | ~0.5s | ~20MB |

---

## 🎯 功能清单

### ✅ 已完成
- [x] 基础框架
- [x] 错误处理
- [x] 响应格式化
- [x] 菜单生成 API
- [x] AI 集成（GLM/OpenAI）
- [x] 降级方案

### ⬜ 待完成
- [ ] 购物清单 API
- [ ] 菜谱查询 API
- [ ] 单元测试
- [ ] 性能测试
- [ ] Docker 支持

---

## 🐛 故障排查

### 编译错误
```bash
# 清理并重新构建
cargo clean
cargo build
```

### 运行时错误
```bash
# 查看详细日志
RUST_LOG=debug cargo run
```

### API 调用失败
1. 检查 .env 文件配置
2. 确认 API Key 正确
3. 检查网络连接
4. 查看日志输出

---

## 📚 相关文档

- [Rust 官方文档](https://www.rust-lang.org/zh-CN/learn)
- [Actix-web 文档](https://actix.rs/)
- [Tokio 文档](https://tokio.rs/)

---

## 🎉 成功标志

当你看到：
```
🚀 启动服务器 0.0.0.0:3000
🤖 AI 提供商: glm
```

**恭喜！Rust 后端已成功运行！** ✅
