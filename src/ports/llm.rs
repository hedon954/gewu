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

/// SMART 目标的结构化分解
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmartGoalDetail {
    /// Specific - 具体的
    pub specific: String,
    /// Measurable - 可衡量的
    pub measurable: String,
    /// Achievable - 可实现的
    pub achievable: String,
    /// Relevant - 相关的
    pub relevant: String,
    /// Time-bound - 有时限的
    pub time_bound: String,
}

/// 评估 SMART 目标的结果
#[derive(Debug, Serialize, Deserialize)]
pub struct SmartGoalVerdict {
    /// 是否通过评估
    pub passed: bool,
    /// 评估分析说明
    pub reason: String,
    /// 改进引导（拒绝时提供）
    #[serde(default)]
    pub guidance: Option<String>,
    /// 优化后的结构化 SMART 目标（通过时提供）
    #[serde(default)]
    pub refined_goal: Option<SmartGoalDetail>,
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

    /// 军师模式：评估并优化 SMART 目标
    async fn evaluate_smart_goal(
        &mut self,
        topic: &str,
        motivation: &str,
        goal: &str,
    ) -> Result<SmartGoalVerdict>;

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
        ui.print_motivation_approved(topic, &self.reason, &self.recommendation);
    }

    pub fn print_rejected(&self) {
        use crate::cli::ui::UI;
        let ui = UI::new();
        ui.print_motivation_rejected(&self.reason, &self.recommendation);
    }
}
