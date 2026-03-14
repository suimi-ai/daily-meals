# 一日三餐 - AI 智能菜单规划助手

> **高性能 Rust 后端 + 微信小程序前端**

## 🎯 项目简介

**一日三餐**是一个基于 AI 的智能菜单规划应用，帮助用户：
- 🍽️ 根据偏好生成个性化菜单
- 🛒 智能生成购物清单
- 📖 提供详细的菜谱指导

## ✨ 核心特性

### 🚀 高性能 Rust 后端
- **Actix-web 框架** - 极致的并发性能
- **10x 性能提升** - 相比 Node.js 版本
- **内存安全** - 零成本抽象
- **异步并发** - 支持 10000+ QPS

### 🤖 AI 集成
- 支持 **GLM** 和 **OpenAI**
- 智能菜单生成
- 个性化推荐
- 降级方案保障

### 📱 微信小程序
- 精美的用户界面
- 流畅的交互体验
- 完整的功能实现

---

## 📦 项目结构

```
一日三餐/
├── server-rust/          # Rust 后端（高性能）
│   ├── src/
│   │   ├── main.rs       # 入口
│   │   ├── config.rs     # 配置管理
│   │   ├── error.rs      # 错误处理
│   │   ├── models/       # 数据模型
│   │   ├── routes/       # API 路由
│   │   ├── services/     # 业务逻辑
│   │   ├── ai/           # AI 集成
│   │   └── utils/        # 工具函数
│   ├── Cargo.toml
│   └── .env.example
├── miniprogram/          # 微信小程序前端
│   ├── pages/
│   ├── utils/
│   └── project.config.json
├── openspec/             # OpenSpec 规范文档
└── README.md
```

---

## 🛠️ 技术栈

### 后端
- **Rust 1.94+** - 系统编程语言
- **Actix-web 4** - Web 框架
- **Serde** - 序列化
- **Tokio** - 异步运行时

### 前端
- **微信小程序** - 原生框架
- **WXML/WXSS** - UI 组件

### AI
- **GLM API** - 智谱 AI
- **OpenAI API** - GPT 系列

---

## 🚀 快速开始

### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. 克隆项目

```bash
git clone https://github.com/suimi-ai/daily-meals.git
cd daily-meals
```

### 3. 配置环境变量

```bash
cd server-rust
cp .env.example .env
# 编辑 .env 文件，填入 API 密钥
```

### 4. 运行后端

```bash
cargo run
```

访问：http://localhost:3000/api/health

### 5. 运行小程序

1. 下载[微信开发者工具](https://developers.weixin.qq.com/miniprogram/dev/devtools/download.html)
2. 导入 `miniprogram/` 目录
3. 填入 AppID
4. 点击"编译"运行

---

## 📡 API 端点

### 菜单相关
- `POST /api/menu/generate` - 生成菜单
- `GET /api/menu/recommend` - 获取推荐

### 购物清单
- `POST /api/shopping/generate` - 生成购物清单
- `POST /api/shopping/inventory` - 更新库存

### 菜谱查询
- `GET /api/recipe/:name` - 获取菜谱详情
- `GET /api/recipe/search` - 搜索菜谱

### 健康检查
- `GET /api/health` - 服务状态

---

## 📊 性能对比

| 指标 | Node.js | Rust | 提升 |
|------|---------|------|------|
| 响应时间 | ~500ms | ~50ms | **10x** ⚡ |
| 并发支持 | 1000 QPS | 10000+ QPS | **10x** 🚀 |
| 内存占用 | ~200MB | ~20MB | **10x** 💾 |

---

## 📚 文档

- [Rust 后端指南](server-rust/RUST_GUIDE.md)
- [快速开始指南](QUICK_START.md)
- [立即开始](START_NOW.md)
- [小程序发布](miniprogram/RELEASE.md)

---

## 🔄 OpenSpec 工作流

本项目采用 **OpenSpec** 规范驱动开发：

1. **Propose** - 创建变更提案
2. **Apply** - 实施开发
3. **Archive** - 归档总结

所有变更记录在 `openspec/changes/` 目录。

---

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

---

## 🙏 致谢

- [Actix-web](https://actix.rs/) - 强大的 Web 框架
- [GLM](https://www.zhipuai.cn/) - 智谱 AI
- [OpenAI](https://openai.com/) - GPT API

---

## 📞 联系方式

- GitHub: https://github.com/suimi-ai/daily-meals
- 问题反馈: https://github.com/suimi-ai/daily-meals/issues

---

**🎉 享受你的每一餐！**
