use anyhow::Result;
use async_trait::async_trait;

use crate::domain::models::Task;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create_task(&self, topic: &str, motivation: &str) -> Result<Task>;

    #[allow(dead_code)]
    async fn get_task(&self, id: i64) -> Result<Task>;
}
