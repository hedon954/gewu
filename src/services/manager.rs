use crate::{
    domain::{
        models::{Record, Task},
        state::TaskStatus,
    },
    ports::{
        llm::{LlmClient, SmartGoalVerdict},
        repository::Repository,
    },
};
use anyhow::Result;

/// The manager of the task.
pub struct TaskManager<LLM: LlmClient, R: Repository> {
    pub llm: LLM,
    pub repo: R,
}

impl<LLM: LlmClient, R: Repository> TaskManager<LLM, R> {
    pub fn new(llm: LLM, repo: R) -> Self {
        Self { llm, repo }
    }

    /// Create a new learning task.
    pub async fn create_task(&mut self, topic: &str, motivation: &str) -> Result<i64> {
        if topic.trim().is_empty() {
            anyhow::bail!("Topic cannot be empty");
        }

        let verdict = self.llm.audit_motivation(topic, motivation).await?;
        if !verdict.passed {
            verdict.print_rejected();
            anyhow::bail!("Motivation rejected");
        }

        let task = self.repo.create_task(topic, motivation).await?;
        verdict.print_passed(topic);
        Ok(task.id)
    }

    /// Evaluate the user's SMART goal against the topic and motivation.
    pub async fn evaluate_smart_goal(
        &mut self,
        id: i64,
        smart_goal: &str,
    ) -> Result<SmartGoalVerdict> {
        let task = self.repo.get_task(id).await?;
        match task {
            None => anyhow::bail!("Task #{} not found", id),
            Some(task) => {
                let verdict = self
                    .llm
                    .evaluate_smart_goal(
                        &task.topic,
                        &task.motivation.unwrap_or_default(),
                        smart_goal,
                    )
                    .await?;
                Ok(verdict)
            }
        }
    }

    /// Update the smart goal of a task.
    pub async fn update_task_smart_goal(&mut self, id: i64, smart_goal: &str) -> Result<()> {
        self.repo.update_task_smart_goal(id, smart_goal).await?;
        Ok(())
    }

    /// Get a task by id
    pub async fn get_task(&self, id: i64) -> Result<Option<Task>> {
        let task = self.repo.get_task(id).await?;
        Ok(task)
    }

    /// Get tasks by status, supports multiple statuses
    pub async fn get_tasks_by_status(&self, status: &[TaskStatus]) -> Result<Vec<Task>> {
        let tasks = self.repo.get_tasks_by_status(status).await?;
        Ok(tasks)
    }

    /// Delete a task by id
    pub async fn delete_task(&mut self, id: i64) -> Result<()> {
        self.repo.delete_task(id).await?;
        Ok(())
    }

    /// Match the learning record with the tasks
    pub async fn match_record_with_tasks(&mut self, record: &str) -> Result<Vec<i64>> {
        let tasks = self.get_tasks_by_status(&[TaskStatus::Active]).await?;
        self.llm.match_tasks(&tasks, record).await
    }

    /// Record the learning progress for the given tasks
    pub async fn record_learning_progress(&mut self, task_ids: &[i64], record: &str) -> Result<()> {
        let record = self.repo.create_record(record).await?;
        for task_id in task_ids {
            self.repo.create_task_record(*task_id, record.id).await?;
        }
        Ok(())
    }

    /// Get the learning records for the given task
    pub async fn get_task_records(&self, task_id: i64) -> Result<Vec<Record>> {
        self.repo.get_task_records(task_id).await
    }

    /// Stream generate a guide for the given task and records
    pub async fn generate_guide_stream(
        &mut self,
        task: &Task,
        records: &[Record],
    ) -> Result<tokio::sync::mpsc::Receiver<String>> {
        self.llm.generate_guide_stream(task, records).await
    }
}
