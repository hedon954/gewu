use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "PascalCase")]
pub enum TaskStatus {
    Validating, // 正在 AI 审核
    Planning,   // 正在制定计划
    Active,     // 进行中
    Reviewing,  // 考核中
    Completed,  // 完成
    Discarded,  // 废弃
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for TaskStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Validating" => Self::Validating,
            "Planning" => Self::Planning,
            "Active" => Self::Active,
            "Reviewing" => Self::Reviewing,
            "Completed" => Self::Completed,
            "Discarded" => Self::Discarded,
            _ => panic!("Invalid task status: {}", value),
        }
    }
}
