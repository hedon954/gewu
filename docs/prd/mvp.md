# Gewu (格物)

## 1. 产品愿景 (Vision)

Gewu 是一个运行在终端（Terminal）里的**反人性**学习管理工具。
它不是为了帮你“记录”待办事项，而是为了帮你**拦截**无效的冲动，**逼迫**你厘清学习动机，并用苏格拉底式的方式**验证**你的学习成果。它是一个拥有“毒舌导师”人格的 AI Agent。

## 2. 核心用户流程 (The Gewu Loop)

用户与 Gewu 的交互遵循严格的**有限状态机 (FSM)** 流转：

1. **发起 (Initiation):** 用户提出想学 `X`。
2. **格物 (Interrogation):** AI 拷问动机（Why）。动机不纯 -> **直接丢弃**。
3. **立志 (Planning):** AI 协助制定 SMART 目标。不具体 -> 打回重写。
4. **习得 (Learning):** AI 生成第一性原理视角的预习摘要（Primer）。
5. **考校 (Examination):** 学完后，AI 出题（3 道情境题）。答不上来 -> **禁止完成**。
6. **归档 (Archiving):** 通过考核，生成总结，计入技能树。

## 3. 功能需求 (Functional Requirements)

基于纯 Rust 架构，MVP 阶段包含以下核心指令：

### 3.1 核心指令集 (CLI Commands)

| 指令               | 描述       | 交互逻辑                                                                 |
| ------------------ | ---------- | ------------------------------------------------------------------------ |
| `gewu add <topic>` | 新增学习项 | 触发交互式问答，输入 `motivation`。系统调用 LLM 判定 Pass/Reject。       |
| `gewu list`        | 查看看板   | 列出当前 `Active` (上限3个) 和 `Planning` 的任务状态。                   |
| `gewu plan <id>`   | 制定计划   | 针对通过验证的任务，进行多轮对话，确定 SMART 目标。并将状态转为 `Active` |
| `gewu learn <id>`  | 开始学习   | 打印 AI 生成的“预习摘要 (Primer)”。                                      |
| `gewu commit <id>` | 提交成果   | 用户输入学习总结。系统进入“考校模式”，生成 3 个问题。                    |
| `gewu review <id>` | 回答考题   | 用户回答 AI 的考题。AI 评分，决定是 Pass 还是 Retry。                    |

### 3.2 AI 人格设定 (System Prompts)

为了体现“格物”的精神，不同的状态调用不同的 System Prompt：

- **守门人 (The Gatekeeper):** 用于 `add` 阶段。尖酸刻薄，极度理性。拒绝一切“为了学而学”的理由。
- **军师 (The Strategist):** 用于 `plan` 阶段。精通敏捷管理，强迫用户将目标量化（SMART 原则）。
- **夫子 (The Master):** 用于 `learn` 阶段。博古通今，擅长用类比和底层原理（如操作系统、编译原理）解释新概念。
- **考官 (The Examiner):** 用于 `review` 阶段。刁钻，喜欢考察边界条件（Corner Cases）和错误处理。

## 4. 技术架构 (Technical Architecture)

采用 **Monolithic Rust CLI** (单体 Rust 命令行应用) 架构。无后端服务，直接直连 LLM API。

### 4.1 技术栈 (Tech Stack)

- **Language:** Rust (Stable)
- **Interface:** CLI / TUI
- `clap`: 命令行参数解析。
- `dialoguer`: 终端交互（输入框、确认框、多行文本编辑）。
- `console`: 终端彩色输出与格式化。

- **Data & Storage:**
- `sqlite` + `sqlx`: 本地轻量级数据库（存储 `.gewu/data.db`）。
- _Schema 设计见下文。_

- **AI Integration:**
- `reqwest`: HTTP Client。
- `serde_json`: 处理 OpenAI/DeepSeek 格式的 JSON。

- **Config:**
- `dotenv`: 管理 API KEY。

### 4.2 数据库设计 (Schema)

仅需两张核心表：

```sql
-- 任务主表
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topic TEXT NOT NULL,                -- 学习主题
    motivation TEXT,                    -- 初始动机
    smart_goal TEXT,                    -- 最终协商的 SMART 目标
    status TEXT NOT NULL,               -- VALIDATING, PLANNING, ACTIVE, REVIEWING, COMPLETED, DISCARDED
    primer_content TEXT,                -- AI 生成的预习摘要
    final_notes TEXT,                   -- 用户提交的总结笔记
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 考核记录表 (一对多)
CREATE TABLE reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    question TEXT NOT NULL,             -- AI 出的题
    user_answer TEXT,                   -- 用户的回答
    ai_feedback TEXT,                   -- AI 的评价
    is_passed BOOLEAN DEFAULT FALSE,    -- 是否通过
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(task_id) REFERENCES tasks(id)
);

```

## 5. MVP 开发计划 (Development Roadmap)

为了本周末能跑通，我们将 MVP 分为三个里程碑：

### Milestone 1: The Gatekeeper (核心验证流) - **本周末目标**

- **目标:** 完成 `gewu add` 和 `gewu list`。
- **实现:**

1. 初始化 Rust 项目与 SQLite 连接。
2. 实现 LLM API 调用模块（封装 DeepSeek API）。
3. 编写“毒舌守门人”的 Prompt。
4. 跑通：输入动机 -> AI 审核 -> 存库（Active 或 Discarded）。

### Milestone 2: The Planner & Learner (计划与学习)

- **目标:** 完成 `gewu plan` 和 `gewu learn`。
- **实现:**

1. 实现多轮对话逻辑（用于协商 SMART 目标）。
2. 实现“预习摘要”生成逻辑。

### Milestone 3: The Examiner (考核闭环)

- **目标:** 完成 `gewu commit` 和 `gewu review`。
- **实现:**

1. 设计问答表结构。
2. 实现 AI 评分逻辑。

---

## 6. 交互示例 (Mockup)

```bash
$ gewu add "Tokio Runtime"

━━━ 🏮 Project Gewu: Cognitive Gatekeeper ━━━

[?] 请陈述你的学习动机 (Why do you want to learn this?):
> 大家都说 Rust 必须学 Tokio，为了以后找工作方便。

🤖 Gatekeeper Thinking...

❌ [REJECTED] 驳回。
理由：典型的随大流心态。"找工作方便"过于宽泛，且未结合你当前的 Java/Go 背景。
除非你能说明 Tokio 解决了具体什么并发问题，或者对比 Go GMP 有何优劣，否则不予通过。

---

$ gewu add "Tokio Runtime"

[?] 请陈述你的学习动机:
> 我想弄懂 Rust 这种无栈协程和 Go 的有栈协程在任务调度上的本质区别，并尝试用 Tokio 手写一个 Redis 简易版。

🤖 Gatekeeper Thinking...

✅ [PASSED] 通过。
理由：动机明确，涉及底层原理对比（无栈 vs 有栈）及具体的工程落地（Redis Clone）。
状态已更新为: PLANNING。请尽快执行 `gewu plan` 制定指标。

```
