use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BugReport {
    pub unid: Uuid,
    pub bug_type: String,
    pub message: String,
    pub user_login: Option<String>,
    pub created_at: OffsetDateTime,
}
