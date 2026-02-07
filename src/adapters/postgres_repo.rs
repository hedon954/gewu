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
        let task: Task = sqlx::query_as(
            r#"
            INSERT INTO tasks (topic, motivation, status)
            VALUES ($1, $2, $3)
            RETURNING id, topic, motivation, smart_goal, status, created_at, updated_at;
            "#,
        )
        .bind(topic)
        .bind(motivation)
        .bind(TaskStatus::Planning)
        .fetch_one(&self.pool)
        .await?;

        Ok(task.into())
    }

    async fn update_task_smart_goal(&self, id: i64, smart_goal: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE tasks SET smart_goal = $1, updated_at = $2, status = $3 WHERE id = $4
            "#,
        )
        .bind(smart_goal)
        .bind(chrono::Utc::now())
        .bind(TaskStatus::Active)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_task(&self, id: i64) -> Result<Option<crate::domain::models::Task>> {
        let task: Option<Task> = sqlx::query_as(
            r#"
            SELECT id, topic, motivation, smart_goal, status, created_at, updated_at FROM tasks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(task.map(Task::into))
    }

    async fn get_tasks_by_status(
        &self,
        status: &[TaskStatus],
    ) -> Result<Vec<crate::domain::models::Task>> {
        let tasks: Vec<Task> = sqlx::query_as(
            r#"
            SELECT id, topic, motivation, smart_goal, status, created_at, updated_at FROM tasks
            WHERE status = ANY($1)
            ORDER BY updated_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks.into_iter().map(Task::into).collect())
    }

    async fn delete_task(&self, id: i64) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM tasks WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx_db_tester::TestPg;

    use super::*;

    #[tokio::test]
    async fn test_create_task() {
        let (repo, _test_pg) = test_repo().await;

        let task = repo.create_task("test", "test").await.unwrap();
        assert_eq!(task.topic, "test".to_string());
        assert_eq!(task.motivation, Some("test".to_string()));
        assert_eq!(task.status, TaskStatus::Planning);
        assert!(task.created_at.timestamp() > 0);
        assert!(task.updated_at.timestamp() > 0);

        let task = repo.get_task(task.id).await.unwrap().unwrap();
        assert_eq!(task.topic, "test".to_string());
        assert_eq!(task.motivation, Some("test".to_string()));
        assert_eq!(task.status, TaskStatus::Planning);
    }

    #[tokio::test]
    async fn test_update_task_smart_goal() {
        let (repo, _test_pg) = test_repo().await;
        let task = repo.create_task("test", "test").await.unwrap();

        repo.update_task_smart_goal(task.id, "smart goal")
            .await
            .unwrap();
        let task = repo.get_task(task.id).await.unwrap().unwrap();
        assert_eq!(task.smart_goal, Some("smart goal".to_string()));
    }

    async fn test_repo() -> (PostgresRepo, TestPg) {
        let pg = test_pg();
        (PostgresRepo::new(pg.get_pool().await), pg)
    }

    fn test_pg() -> TestPg {
        TestPg::new(
            "postgres://gewu_user:gewu_pass@localhost:5532/gewu".to_string(),
            std::path::Path::new("./migrations"),
        )
    }
}
