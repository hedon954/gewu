use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::state::TaskStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub topic: String,
    pub motivation: Option<String>,
    pub smart_goal: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
