# Rust 安装指南

## 快速安装

### macOS / Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 验证安装
```bash
rustc --version
cargo --version
```

### 安装完成后
```bash
cd ~/一日三餐/server-rust
cargo build
cargo run
```

## 国内镜像加速（可选）

编辑 `~/.cargo/config`:
```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

## 完整文档
https://www.rust-lang.org/zh-CN/tools/install
