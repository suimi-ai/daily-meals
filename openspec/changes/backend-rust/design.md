# 技术设计

## 架构设计

### 项目结构
```
server-rust/
├── Cargo.toml              # 项目配置
├── .env                    # 环境变量
├── src/
│   ├── main.rs            # 入口文件
│   ├── config.rs          # 配置管理
│   ├── error.rs           # 错误定义
│   ├── routes/            # 路由
│   │   ├── mod.rs
│   │   ├── menu.rs
│   │   ├── shopping.rs
│   │   └── recipe.rs
│   ├── controllers/       # 控制器
│   │   ├── mod.rs
│   │   ├── menu.rs
│   │   ├── shopping.rs
│   │   └── recipe.rs
│   ├── services/          # 业务逻辑
│   │   ├── mod.rs
│   │   ├── menu.rs
│   │   ├── shopping.rs
│   │   └── recipe.rs
│   ├── models/            # 数据模型
│   │   ├── mod.rs
│   │   ├── dish.rs
│   │   ├── ingredient.rs
│   │   └── recipe.rs
│   ├── ai/                # AI 集成
│   │   ├── mod.rs
│   │   ├── glm.rs
│   │   └── openai.rs
│   └── utils/             # 工具函数
│       ├── mod.rs
│       └── response.rs
└── openapi/               # OpenAPI 规范（复用）
```

## 核心设计

### 1. 错误处理

```rust
// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("AI service error: {0}")]
    AIServiceError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_code = match self {
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::NotFoundError(_) => "NOT_FOUND",
            AppError::AIServiceError(_) => "AI_SERVICE_ERROR",
            AppError::InternalError(_) => "INTERNAL_ERROR",
        };

        let status_code = match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            success: false,
            error: ErrorDetail {
                code: error_code.to_string(),
                message: self.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            },
        })
    }
}
```

### 2. 响应格式

```rust
// src/utils/response.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub timestamp: String,
}

pub fn success<T: Serialize>(data: T) -> SuccessResponse<T> {
    SuccessResponse {
        success: true,
        data,
        message: None,
    }
}

pub fn success_with_message<T: Serialize>(data: T, msg: &str) -> SuccessResponse<T> {
    SuccessResponse {
        success: true,
        data,
        message: Some(msg.to_string()),
    }
}
```

### 3. AI 服务抽象

```rust
// src/ai/mod.rs
use async_trait::async_trait;
use crate::models::Menu;

#[async_trait]
pub trait AIService: Send + Sync {
    async fn generate_menu(&self, params: MenuParams) -> Result<Menu, AppError>;
}

pub struct GLMService {
    api_key: String,
    client: reqwest::Client,
}

pub struct OpenAIService {
    api_key: String,
    client: reqwest::Client,
}

// 工厂模式选择 AI 提供商
pub fn create_ai_service(provider: &str, api_key: &str) -> Box<dyn AIService> {
    match provider {
        "glm" => Box::new(GLMService::new(api_key)),
        "openai" => Box::new(OpenAIService::new(api_key)),
        _ => panic!("Unsupported AI provider"),
    }
}
```

### 4. 路由设计

```rust
// src/main.rs
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api")
                    .route("/menu/generate", web::post().to(menu::generate))
                    .route("/menu/recommend", web::get().to(menu::recommend))
                    .route("/shopping/generate", web::post().to(shopping::generate))
                    .route("/recipe/{name}", web::get().to(recipe::get))
            )
            .route("/health", web::get().to(health))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
```

## 性能优化

### 1. 连接池
```rust
// HTTP 客户端连接池
let client = reqwest::Client::builder()
    .pool_max_idle_per_host(10)
    .pool_idle_timeout(Duration::from_secs(30))
    .build()?;
```

### 2. 异步 I/O
```rust
// 使用 Tokio 异步文件 I/O
let content = tokio::fs::read_to_string("data.json").await?;
```

### 3. 缓存
```rust
// 使用 moka 缓存库
use moka::future::Cache;

let cache = Cache::builder()
    .max_capacity(1000)
    .time_to_live(Duration::from_secs(300))
    .build();
```

## 依赖清单

```toml
[dependencies]
actix-web = "4"
actix-cors = "0.6"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.11", features = ["json"] }
thiserror = "1"
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
dotenv = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
async-trait = "0.1"
```

## 部署优化

### 1. 编译优化
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### 2. Docker 支持
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/daily-meals /usr/local/bin/
CMD ["daily-meals"]
```

## 迁移策略

### 阶段 1：基础框架（第1天）
- 创建项目结构
- 实现错误处理
- 实现响应格式
- 配置管理

### 阶段 2：核心 API（第2-3天）
- 实现菜单生成 API
- 实现购物清单 API
- 实现菜谱查询 API

### 阶段 3：AI 集成（第4天）
- 集成 GLM API
- 集成 OpenAI API
- 降级方案

### 阶段 4：测试和优化（第5天）
- 单元测试
- 性能测试
- 优化调优

## 性能对比预期

| 指标 | Node.js | Rust | 提升 |
|------|---------|------|------|
| 响应时间 | ~500ms | ~50ms | 10x |
| 并发数 | 1000 | 10000+ | 10x |
| 内存占用 | ~200MB | ~30MB | 6x |
| CPU 使用率 | 80% | 20% | 4x |

## 风险和缓解

### 风险 1：学习曲线
**缓解**: 使用熟悉的架构模式，保持与 Node.js 版本相似

### 风险 2：AI SDK 不成熟
**缓解**: 直接使用 HTTP API 调用，不依赖 SDK

### 风险 3：开发时间较长
**缓解**: 分阶段实施，先实现核心功能
