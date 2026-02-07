# Gewu

致知在格物，物格而后知至。

## 技术栈

- **Rust**: 主要编程语言
- **PostgreSQL**: 数据存储
- **SQLx**: 数据库交互
- **DeepSeek API**: AI 能力

## 环境准备

### 1. 安装 PostgreSQL

使用 Docker Compose（推荐）:

```bash
cd scripts
docker compose up -d
```

或手动安装 PostgreSQL 16+

### 2. 配置环境变量

复制配置模板：

```bash
cp .env.example .env
```

编辑 `.env` 文件，设置你的配置：

```env
DATABASE_URL=postgresql://gewu_user:gewu_pass@localhost:5432/gewu
DEEPSEEK_API_KEY=your_api_key_here
```

### 3. 安装 SQLx CLI 并运行迁移

```bash
# 安装 SQLx CLI
cargo install sqlx-cli --features postgres

# 运行数据库迁移
sqlx migrate run
```

## 开发

```bash
# 构建项目
cargo build

# 运行测试
cargo test

# 运行程序
cargo run
```

## 架构

本项目采用六边形架构（Hexagonal Architecture）+ DDD（领域驱动设计）简化版：

- **领域层 (Domain)**: 核心业务逻辑和模型
- **端口层 (Ports)**: 定义外部依赖的接口
- **适配器层 (Adapters)**: 实现具体的技术细节
- **应用层 (Services)**: 协调业务流程
- **展示层 (CLI)**: 用户交互界面

详细技术方案见 [docs/adr/mvp.md](docs/adr/mvp.md)
