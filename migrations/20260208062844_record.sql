-- Add migration script here
CREATE TABLE IF NOT EXISTS records (
    id BIGSERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_records_created_at ON records (created_at);

CREATE TABLE IF NOT EXISTS task_records (
    id BIGSERIAL PRIMARY KEY,
    task_id BIGINT NOT NULL,
    record_id BIGINT NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
    FOREIGN KEY (record_id) REFERENCES records (id) ON DELETE CASCADE
);

CREATE INDEX idx_task_records_task_id ON task_records (task_id);

CREATE INDEX idx_task_records_record_id ON task_records (record_id);