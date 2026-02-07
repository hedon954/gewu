use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 审核动机的结果，包含是否通过以及原因说明
#[derive(Debug, Serialize, Deserialize)]
pub struct GatekeeperVerdict {
    /// 是否通过审核
    pub passed: bool,
    /// 审核原因说明
    pub reason: String,
    /// 审核建议
    pub recommendation: String,
}

/// AI 客户端接口，定义了所有 AI 交互的抽象方法
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// 守门人模式：审核动机
    async fn audit_motivation(
        &mut self,
        topic: &str,
        motivation: &str,
    ) -> Result<GatekeeperVerdict>;

    // /// 军师模式：优化 SMART 目标
    // async fn refine_plan(&self, goal: &str) -> Result<String>;

    // /// 夫子模式：生成预习摘要
    // async fn generate_primer(&self, goal: &str) -> Result<String>;

    // /// 考官模式：生成考题
    // async fn generate_questions(&self, goal: &str) -> Result<Vec<String>>;

    // /// 考官模式：评分
    // async fn score_answer(&self, question: &str, answer: &str) -> Result<f32>;
}

impl GatekeeperVerdict {
    pub fn print_passed(&self, topic: &str) {
        use crate::cli::ui::UI;
        let ui = UI::new();
        ui.print_approved(topic, &self.reason, &self.recommendation);
    }

    pub fn print_rejected(&self) {
        use crate::cli::ui::UI;
        let ui = UI::new();
        ui.print_rejected(&self.reason, &self.recommendation);
    }
}
