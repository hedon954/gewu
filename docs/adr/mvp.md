# Gewu MVP 技术方案

为了确保代码的可维护性、可扩展性以及符合 Rust 的工程最佳实践，我们将采用 **“领域驱动设计 (DDD) 简化版”** 配合 **“六边形架构 (Hexagonal Architecture)”** 的思想。虽然这只是一个 CLI 工具，但清晰的分层能让你在未来轻松替换 LLM 供应商或迁移数据库，同时这也是学习 Rust 架构模式的绝佳机会。

### 1. 总体架构图 (System Architecture)

我们将应用划分为四个层次，依赖关系严格由外向内（外部依赖内部，内部不依赖外部）：

```mermaid
graph TD
    User((用户)) --> CLI[展示层: CLI/TUI]
    CLI --> App[应用层: Use Cases]
    App --> Domain[领域层: Models & State]

    App --> Infra[基础设施层: Impl]
    Infra --实现--> App

    subgraph "Infrastructure (Adapters)"
        DB[(PostgreSQL)]
        AI[DeepSeek API]
    end

    Infra --> DB
    Infra --> AI

```

### 2. 项目目录结构 (Project Layout)

```text
gewu/
├── Cargo.toml          # 依赖管理
├── .env                # 环境变量 (API Keys, DB URL)
├── migrations/         # PostgreSQL 数据库迁移脚本
│   └── 20260207_init.sql
├── src/
│   ├── main.rs         # 程序入口 (Entry Point)
│   ├── config.rs       # 配置加载 (Config)
│   ├── error.rs        # 统一错误处理 (Error Handling)
│   ├── domain/         # [核心] 领域模型 (纯 Rust 逻辑)
│   │   ├── mod.rs
│   │   ├── models.rs   # Task, Review 结构体
│   │   └── state.rs    # TaskStatus 状态机定义
│   ├── ports/          # [接口] 定义外部依赖的 Trait (抽象层)
│   │   ├── mod.rs
│   │   ├── repository.rs # 数据库操作接口
│   │   └── llm.rs        # AI 交互接口
│   ├── adapters/       # [适配器] 接口的具体实现
│   │   ├── mod.rs
│   │   ├── postgres_repo.rs # 基于 SQLx 的 PostgreSQL 实现
│   │   └── deepseek.rs      # 基于 Reqwest 的 DeepSeek 实现
│   ├── services/       # [应用] 业务逻辑流 (Use Cases)
│   │   ├── mod.rs
│   │   ├── manager.rs     # 协调领域对象和适配器
│   │   └── prompts.rs     # AI 提示词管理
│   └── cli/            # [交互] 命令行解析与交互
│       ├── mod.rs
│       ├── commands.rs    # Clap 子命令定义
│       └── ui.rs          # Dialoguer 交互逻辑

```

---

### 3. 核心模块详细设计

#### 3.1 领域层 (Domain Layer)

这是系统的心脏，不依赖任何外部库（如数据库或 HTTP 客户端），只包含纯 Rust 数据结构和业务规则。

**`src/domain/state.rs` (核心状态机)**
利用 Rust 强大的枚举类型来定义 FSM。

```rust
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Validating,     // 正在 AI 审核
    Planning,       // 正在制定计划
    Active,         // 进行中
    Reviewing,      // 考核中
    Completed,      // 完成
    Discarded,      // 废弃
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// 可以在这里定义状态流转规则，防止非法跳转
impl TaskStatus {
    pub fn can_transition_to(&self, next: &TaskStatus) -> bool {
        match (self, next) {
            (Self::Validating, Self::Planning) => true,
            (Self::Validating, Self::Discarded) => true,
            (Self::Planning, Self::Active) => true,
            // ... 其他规则
            _ => false,
        }
    }
}

```

**`src/domain/models.rs`**
定义核心实体。

```rust
use super::state::TaskStatus;
use chrono::{DateTime, Utc};

pub struct Task {
    pub id: i64,
    pub topic: String,
    pub motivation: Option<String>,
    pub smart_goal: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

```

#### 3.2 端口层 (Ports Layer)

定义 Trait，实现依赖倒置。这使得我们可以在测试时 Mock 掉数据库和 AI，或者在未来把 DeepSeek 换成 OpenAI 而不需要修改核心逻辑。

**`src/ports/llm.rs`**

```rust
use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug)]
pub struct GatekeeperVerdict {
    pub passed: bool,
    pub reason: String,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    /// 守门人模式：审核动机
    async fn audit_motivation(&self, topic: &str, motivation: &str) -> Result<GatekeeperVerdict>;

    /// 军师模式：优化 SMART 目标
    async fn refine_plan(&self, goal: &str) -> Result<String>;

    // ... 其他接口
}

```

#### 3.3 基础设施层 (Adapters Layer)

这里是“干脏活”的地方，处理具体的 SQL 和 HTTP 请求。

**`src/adapters/postgres_repo.rs`**
使用 `sqlx` 实现持久化。

```rust
use crate::domain::models::Task;
use crate::domain::state::TaskStatus;
use sqlx::PgPool;
use anyhow::Result;

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_task(&self, topic: &str, motivation: &str) -> Result<Task> {
        let status = TaskStatus::Validating;
        // sqlx query_as! 宏提供了编译时 SQL 检查
        let rec = sqlx::query!(
            r#"
            INSERT INTO tasks (topic, motivation, status)
            VALUES ($1, $2, $3)
            RETURNING id, topic, motivation, smart_goal, status, created_at, updated_at
            "#,
            topic, motivation, status as TaskStatus
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Task {
            id: rec.id,
            topic: rec.topic,
            // ... 转换逻辑
            status: rec.status, // 需要实现 sqlx::Type trait 或手动转换
            // ...
        })
    }
}

```

**`src/adapters/deepseek.rs`**
AI 客户端实现。

```rust
use crate::ports::llm::{LlmClient, GatekeeperVerdict};
use reqwest::Client;
// ...

pub struct DeepSeekClient {
    client: Client,
    api_key: String,
    base_url: String,
}

// impl LlmClient for DeepSeekClient ...

```

#### 3.4 应用层 (Application Layer)

协调领域对象和适配器。

**`src/services/manager.rs`**

```rust
use crate::domain::models::Task;
use crate::ports::repository::Repository;
use anyhow::Result;

/// Service struct wrapping a repository, providing business logic.
pub struct TaskManager<R: Repository> {
    pub repo: R,
}

impl<R: Repository> TaskManager<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Create a new learning task via repository, can add validations here.
    pub async fn create_task(&self, topic: &str, motivation: &str) -> Result<Task> {
        // Example business rule: topic must not be empty
        if topic.trim().is_empty() {
            anyhow::bail!("Topic cannot be empty");
        }

        self.repo.create_task(topic, motivation).await
    }

    /// Get a task by id
    pub async fn get_task(&self, id: i64) -> Result<Task> {
        self.repo.get_task(id).await
    }
}
```

#### 3.5 命令行交互层 (CLI Layer)

使用 `clap` 定义命令，`dialoguer` 处理交互。

**`src/cli/commands.rs`**

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gewu", about = "Cognitive Gatekeeper", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 新增学习计划 (Initiate a new learning task)
    Add {
        /// 学习主题
        topic: Option<String>,
    },
    /// 查看任务列表
    List,
    // ...
}

```

#### 3.6 数据库表

```sql
CREATE TABLE tasks (
    id BIGSERIAL PRIMARY KEY,
    topic TEXT NOT NULL,
    motivation TEXT,
    smart_goal TEXT,
    status TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

```sql
CREATE TABLE reviews (
    id BIGSERIAL PRIMARY KEY,
    task_id BIGINT NOT NULL,
    question TEXT NOT NULL,
    user_answer TEXT,
    ai_feedback TEXT,
    is_passed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    FOREIGN KEY(task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- 创建索引提高查询性能
CREATE INDEX idx_reviews_task_id ON reviews(task_id);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
```

### 4. 关键技术点实现方案

#### 4.1 异步运行时与状态共享

在 `main.rs` 中，我们初始化核心组件，并将其注入到各个 Command Handler 中。为了避免到处传参，可以定义一个 `AppState`。

```rust
// src/main.rs 伪代码

struct AppState {
    repo: PostgresRepository,
    llm: Box<dyn LlmClient>, // 使用 Box<dyn Trait> 实现多态
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // 1. 初始化 DB
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    let repo = PostgresRepository::new(pool);

    // 2. 初始化 AI
    let api_key = std::env::var("DEEPSEEK_API_KEY")?;
    let llm = Box::new(DeepSeekClient::new(api_key));

    let state = AppState { repo, llm };

    // 3. 解析命令
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { topic } => commands::handle_add(state, topic).await?,
        Commands::List => commands::handle_list(state).await?,
        // ...
    }

    Ok(())
}

```

#### 4.2 错误处理 (Error Handling)

使用 `thiserror` 定义领域错误，使用 `anyhow` 处理顶层应用错误。

**`src/error.rs`**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("AI Service error: {0}")]
    AiError(String),

    #[error("Invalid state transition: Cannot move from {0} to {1}")]
    InvalidTransition(String, String),
}

```

#### 4.3 数据库迁移 (Migrations)

Rust 的 `sqlx` 支持编译时迁移。

1. 安装工具：`cargo install sqlx-cli --features postgres`
2. 创建数据库：`createdb gewu` (或通过 Docker 启动 PostgreSQL)
3. 设置环境变量：
   ```bash
   export DATABASE_URL="postgresql://username:password@localhost:5432/gewu"
   ```
4. 创建迁移文件：`sqlx migrate add init`
5. 运行迁移：`sqlx migrate run`

**推荐使用 Docker Compose 管理 PostgreSQL：**

```yaml
# docker-compose.yml
version: "3.8"
services:
  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: gewu
      POSTGRES_USER: gewu_user
      POSTGRES_PASSWORD: gewu_pass
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

启动命令：`docker-compose up -d`

这保证了用户第一次运行你的工具时，会自动创建 PostgreSQL 表结构。

```

```
