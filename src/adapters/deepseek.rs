use anyhow::Result;
use async_trait::async_trait;
use futures_util::StreamExt;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{
        ChatCompletionMessage, Content, MessageRole,
        chat_completion::ChatCompletionRequest,
        chat_completion_stream::{ChatCompletionStreamRequest, ChatCompletionStreamResponse},
    },
};

use crate::{
    domain::models::{Record, Task},
    ports::llm::{GatekeeperVerdict, LlmClient, MatchTasksResult, SmartGoalVerdict},
    services::prompts::{
        audit_motivation_prompt, evaluate_smart_goal_prompt, generate_guide_prompt,
        match_tasks_prompt,
    },
};

pub struct DeepSeek {
    client: OpenAIClient,
}

impl DeepSeek {
    pub async fn try_new(api_key: String) -> Result<Self> {
        let client = OpenAIClient::builder()
            .with_endpoint("https://api.deepseek.com")
            .with_api_key(api_key)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build OpenAIClient: {}", e))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl LlmClient for DeepSeek {
    async fn audit_motivation(
        &mut self,
        topic: &str,
        motivation: &str,
    ) -> Result<GatekeeperVerdict> {
        let prompt = audit_motivation_prompt(topic, motivation);
        let response = self.client.chat_completion(chat_request(prompt)).await?;

        let content = response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default();
        Ok(serde_json::from_str(&content)?)
    }

    async fn evaluate_smart_goal(
        &mut self,
        topic: &str,
        motivation: &str,
        goal: &str,
    ) -> Result<SmartGoalVerdict> {
        let prompt = evaluate_smart_goal_prompt(topic, motivation, goal);
        let response = self.client.chat_completion(chat_request(prompt)).await?;

        let content = response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default();
        Ok(serde_json::from_str(&content)?)
    }

    async fn match_tasks(&mut self, tasks: &[Task], record: &str) -> Result<Vec<i64>> {
        let task_json_str = serde_json::to_string(tasks)?;
        let prompt = match_tasks_prompt(&task_json_str, record);
        let response = self.client.chat_completion(chat_request(prompt)).await?;

        let content = response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default();
        let result: MatchTasksResult = serde_json::from_str(&content)?;
        Ok(result.task_ids)
    }

    async fn generate_guide_stream(
        &mut self,
        task: &Task,
        records: &[Record],
    ) -> Result<tokio::sync::mpsc::Receiver<String>> {
        let task_json_str = serde_json::to_string(task)?;
        let record_json_str = serde_json::to_string(records)?;
        let prompt = generate_guide_prompt(&task_json_str, &record_json_str);

        let mut stream = self
            .client
            .chat_completion_stream(stream_chat_request(prompt))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to start streaming: {}", e))?;

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(response) = stream.next().await {
                match response {
                    ChatCompletionStreamResponse::Content(content) => {
                        if tx.send(content).await.is_err() {
                            break;
                        }
                    }
                    ChatCompletionStreamResponse::Done => break,
                    _ => {}
                }
            }
        });

        Ok(rx)
    }
}

fn chat_request(content: String) -> ChatCompletionRequest {
    ChatCompletionRequest::new(
        "deepseek-chat".to_string(),
        vec![ChatCompletionMessage {
            role: MessageRole::assistant,
            content: Content::Text(content),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    )
}

fn stream_chat_request(content: String) -> ChatCompletionStreamRequest {
    ChatCompletionStreamRequest::new(
        "deepseek-chat".to_string(),
        vec![ChatCompletionMessage {
            role: MessageRole::assistant,
            content: Content::Text(content),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "skip test that requires third party service"]
    async fn audit_motivation_too_vague_should_reject() {
        let mut deepseek = DeepSeek::try_new(dotenv::var("DEEPSEEK_API_KEY").unwrap())
            .await
            .unwrap();

        let result = deepseek
            .audit_motivation("Rust", "I want to learn Rust")
            .await
            .unwrap();

        println!("{}", serde_json::to_string_pretty(&result).unwrap());

        assert!(!result.passed);
        assert!(!result.reason.is_empty());
        assert!(!result.recommendation.is_empty());
    }

    #[tokio::test]
    #[ignore = "skip test that requires third party service"]
    async fn audit_motivation_addresses_current_pain_point_should_approve() {
        let mut deepseek = DeepSeek::try_new(dotenv::var("DEEPSEEK_API_KEY").unwrap())
            .await
            .unwrap();

        let result = deepseek
            .audit_motivation(
                "building a code review ai agent",
                "I want to build a code review ai agent to improve my code review skills, as well as learn how to build ai agents",
            )
            .await
            .unwrap();

        println!("{}", serde_json::to_string_pretty(&result).unwrap());

        assert!(result.passed);
        assert!(!result.reason.is_empty());
        assert!(!result.recommendation.is_empty());
    }

    #[tokio::test]
    #[ignore = "skip test that requires third party service"]
    async fn evaluate_smart_goal_should_reject_bad_goal() {
        let mut deepseek = DeepSeek::try_new(dotenv::var("DEEPSEEK_API_KEY").unwrap())
            .await
            .unwrap();

        let result = deepseek
            .evaluate_smart_goal(
                "learning Rust",
                "To become a better software engineer for my next project at work.",
                "I want to get better.",
            )
            .await
            .unwrap();

        println!("{}", serde_json::to_string_pretty(&result).unwrap());

        assert!(!result.passed);
        assert!(!result.reason.is_empty());
        assert!(result.guidance.is_some());
        assert!(result.refined_goal.is_none());
    }

    #[tokio::test]
    #[ignore = "skip test that requires third party service"]
    async fn evaluate_smart_goal_should_approve_good_goal() {
        let mut deepseek = DeepSeek::try_new(dotenv::var("DEEPSEEK_API_KEY").unwrap())
            .await
            .unwrap();

        let result = deepseek
            .evaluate_smart_goal(
                "learning Rust",
                "To become a better software engineer for my next project at work.",
                "Within one month, complete the official Rust book and build a small CLI tool to automate part of my workflow, measuring success by completing at least one practical project and passing all end-of-chapter exercises.",
            )
            .await
            .unwrap();

        println!("{}", serde_json::to_string_pretty(&result).unwrap());

        assert!(result.passed);
        assert!(!result.reason.is_empty());
        assert!(result.refined_goal.is_some());

        let detail = result.refined_goal.unwrap();
        assert!(!detail.specific.is_empty());
        assert!(!detail.measurable.is_empty());
        assert!(!detail.achievable.is_empty());
        assert!(!detail.relevant.is_empty());
        assert!(!detail.time_bound.is_empty());
    }
}
