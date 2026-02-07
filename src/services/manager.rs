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
    pub async fn create_task(&mut self, topic: &str, motivation: &str) -> Result<()> {
        if topic.trim().is_empty() {
            anyhow::bail!("Topic cannot be empty");
        }

        let verdict = self.llm.audit_motivation(topic, motivation).await?;
        if !verdict.passed {
            verdict.print_rejected();
            anyhow::bail!("Motivation rejected");
        }

        self.repo.create_task(topic, motivation).await?;
        verdict.print_passed(topic);
        Ok(())
    }
}
