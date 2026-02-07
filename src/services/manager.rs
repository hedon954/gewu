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
    pub async fn create_task(&mut self, topic: &str, motivation: &str) -> Result<String> {
        if topic.trim().is_empty() {
            anyhow::bail!("Topic cannot be empty");
        }

        let verdict = self.llm.audit_motivation(topic, motivation).await?;
        if !verdict.passed {
            anyhow::bail!(verdict.print_rejected());
        }

        let task = self.repo.create_task(topic, motivation).await?;
        println!("{}", serde_json::to_string_pretty(&task)?);
        Ok(verdict.print_passed())
    }
}
