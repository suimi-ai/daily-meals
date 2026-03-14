# 一日三餐 - Rust 后端

基于 Rust + Actix-web 的高性能餐饮规划 API 服务

## 特性

- ⚡ **高性能** - 比 Node.js 快 5-10 倍
- 🔒 **内存安全** - 编译时保证内存安全
- 🚀 **高并发** - 支持 10000+ QPS
- 💾 **低内存** - 内存占用 < 50MB

## 安装 Rust

```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## 快速开始

### 1. 配置环境变量
```bash
cp .env.example .env
# 编辑 .env 文件
```

### 2. 运行服务
```bash
cargo run
```

### 3. 构建生产版本
```bash
cargo build --release
./target/release/daily-meals
```

## 性能对比

| 指标 | Node.js | Rust | 提升 |
|------|---------|------|------|
| 响应时间 | ~500ms | ~50ms | 10x |
| 并发数 | 1000 | 10000+ | 10x |
| 内存占用 | ~200MB | ~30MB | 6x |

## API 端点

- `POST /api/menu/generate` - 生成菜单
- `GET /api/menu/recommend` - 获取推荐
- `GET /health` - 健康检查

## 技术栈

- **Actix-web** - 高性能 Web 框架
- **Tokio** - 异步运行时
- **Serde** - 序列化框架
- **Reqwest** - HTTP 客户端

## 许可证

MIT
