use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::{adapters::models::Task, domain::state::TaskStatus, ports::repository::Repository};

pub struct PostgresRepo {
    pool: PgPool,
}

impl PostgresRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for PostgresRepo {
    async fn create_task(
        &self,
        topic: &str,
        motivation: &str,
    ) -> Result<crate::domain::models::Task> {
        let status = TaskStatus::Validating;
        let task: Task = sqlx::query_as(
            r#"
            INSERT INTO tasks (topic, motivation, status)
            VALUES ($1, $2, $3)
            RETURNING id, topic, motivation, smart_goal, status, created_at, updated_at;
            "#,
        )
        .bind(topic)
        .bind(motivation)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(task.into())
    }

    async fn get_task(&self, id: i64) -> Result<crate::domain::models::Task> {
        let task: Task = sqlx::query_as(
            r#"
            SELECT id, topic, motivation, smart_goal, status, created_at, updated_at FROM tasks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task.into())
    }
}

#[cfg(test)]
mod tests {
    use sqlx_db_tester::TestPg;

    use super::*;

    #[tokio::test]
    async fn test_create_task() {
        let tdb = TestPg::new(
            "postgres://gewu_user:gewu_pass@localhost:5532/gewu".to_string(),
            std::path::Path::new("./migrations"),
        );

        let repo = PostgresRepo::new(tdb.get_pool().await);
        let task = repo.create_task("test", "test").await.unwrap();
        assert_eq!(task.topic, "test".to_string());
        assert_eq!(task.motivation, Some("test".to_string()));
        assert_eq!(task.status, TaskStatus::Validating);
        assert!(task.created_at.timestamp() > 0);
        assert!(task.updated_at.timestamp() > 0);

        let task = repo.get_task(task.id).await.unwrap();
        assert_eq!(task.topic, "test".to_string());
        assert_eq!(task.motivation, Some("test".to_string()));
        assert_eq!(task.status, TaskStatus::Validating);
    }
}
