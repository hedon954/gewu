use crate::ports::{llm::LlmClient, repository::Repository};
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

    /// Execute the smart goal evaluation and update the task if approved.
    pub async fn evaluate_smart_goal(&mut self, id: i64, smart_goal: &str) -> Result<String> {
        let task = self.repo.get_task(id).await?;
        let verdict = self
            .llm
            .evaluate_smart_goal(
                &task.topic,
                &task.motivation.unwrap_or_default(),
                smart_goal,
            )
            .await?;
        if !verdict.passed {
            println!("{}", serde_json::to_string_pretty(&verdict)?);
            anyhow::bail!("Smart goal rejected");
        }
        println!("{}", serde_json::to_string_pretty(&verdict)?);
        Ok(verdict.recommendation)
    }

    /// Update the smart goal of a task.
    pub async fn update_task_smart_goal(&mut self, id: i64, smart_goal: &str) -> Result<()> {
        self.repo.update_task_smart_goal(id, smart_goal).await?;
        println!("Smart goal updated successfully");
        Ok(())
    }
}
