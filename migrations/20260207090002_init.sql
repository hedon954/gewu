-- Add migration script here

-- 创建 tasks 表
CREATE TABLE tasks (
    id BIGSERIAL PRIMARY KEY,
    topic TEXT NOT NULL,
    motivation TEXT,
    smart_goal TEXT,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 创建 reviews 表
CREATE TABLE reviews (
    id BIGSERIAL PRIMARY KEY,
    task_id BIGINT NOT NULL,
    question TEXT NOT NULL,
    user_answer TEXT,
    ai_feedback TEXT,
    is_passed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE
);

-- 创建索引以提高查询性能
CREATE INDEX idx_reviews_task_id ON reviews (task_id);

CREATE INDEX idx_tasks_status ON tasks (status);

CREATE INDEX idx_tasks_created_at ON tasks (created_at);