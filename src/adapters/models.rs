use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::state::TaskStatus;

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub topic: String,
    pub motivation: Option<String>,
    pub smart_goal: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Task> for crate::domain::models::Task {
    fn from(value: Task) -> Self {
        crate::domain::models::Task {
            id: value.id,
            topic: value.topic,
            motivation: value.motivation,
            smart_goal: value.smart_goal,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
