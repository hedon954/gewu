use anyhow::Result;
use async_trait::async_trait;

use crate::domain::models::Task;

#[async_trait]
pub trait Repository: Send + Sync {
    /// Create a new task
    async fn create_task(&self, topic: &str, motivation: &str) -> Result<Task>;

    /// Update the smart goal of a task
    async fn update_task_smart_goal(&self, id: i64, smart_goal: &str) -> Result<()>;

    /// Get a task by id
    #[allow(dead_code)]
    async fn get_task(&self, id: i64) -> Result<Task>;
}
