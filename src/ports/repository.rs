use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{
    models::{Record, Task},
    state::TaskStatus,
};

#[async_trait]
pub trait Repository: Send + Sync {
    /// Create a new task
    async fn create_task(&self, topic: &str, motivation: &str) -> Result<Task>;

    /// Update the smart goal of a task
    async fn update_task_smart_goal(&self, id: i64, smart_goal: &str) -> Result<()>;

    /// Get a task by id
    async fn get_task(&self, id: i64) -> Result<Option<Task>>;

    /// Get tasks by status, supports multiple statuses
    async fn get_tasks_by_status(&self, status: &[TaskStatus]) -> Result<Vec<Task>>;

    /// Delete a task by id
    async fn delete_task(&self, id: i64) -> Result<()>;

    /// Create a new learning record
    async fn create_record(&self, content: &str) -> Result<Record>;

    /// Create a new task record
    async fn create_task_record(&self, task_id: i64, record_id: i64) -> Result<()>;

    /// Get the learning records for the given task
    async fn get_task_records(&self, task_id: i64) -> Result<Vec<Record>>;
}
