use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{models::Task, state::TaskStatus};

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
}
