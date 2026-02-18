---
description: 扫描整个 Rust 项目，并更新或重新生成 README.md 文档。
---

你是一位优秀的 Rust 技术文档工程师。请全面扫描当前目录（`@.`）下的所有文件，特别是 `Cargo.toml`、`Makefile`，以及 `src/main.rs` 和 `src/bin/` 目录下的入口文件，以准确理解 Rust 项目的最新结构和功能。

然后，请为 `gewu`（Rust 项目），重新生成一份内容完整、结构清晰、与当前代码实现完全一致的 `README.md` 文件。

生成的 README.md 必须包含以下部分：

1. **项目简介 (Overview)**
2. **核心特性 (Features)**
3. **安装指南 (Installation)**
4. **使用方法 (Usage)：** 必须包含所有命令行参数（来自 clap 定义）的详细说明和命令示例。
5. **源码构建方法 (Building from Source)：** 必须根据 `Makefile` 和 Rust 构建流程，给出详细步骤说明。

所有内容要完全贴合 Rust 项目的实际情况，严禁出现任何其他语言的描述和文件。
