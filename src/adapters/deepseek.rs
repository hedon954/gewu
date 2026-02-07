use anyhow::Result;
use async_trait::async_trait;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{
        ChatCompletionMessage, Content, MessageRole, chat_completion::ChatCompletionRequest,
    },
};

use crate::{
    ports::llm::{GatekeeperVerdict, LlmClient},
    services::prompts::audit_motivation_prompt,
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
        let response = self
            .client
            .chat_completion(ChatCompletionRequest::new(
                "deepseek-chat".to_string(),
                vec![ChatCompletionMessage {
                    role: MessageRole::assistant,
                    content: Content::Text(prompt),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                }],
            ))
            .await?;

        let content = response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default();
        Ok(serde_json::from_str(&content)?)
    }
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
}
