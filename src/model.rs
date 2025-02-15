use serde::{Deserialize, Serialize}
use chron::{DateTime, Utc}
use sqlx::FromRow;
use uuid:: Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskModel {
    pub id: Uuid,
    pub title:String,
    pub content: STring,
    pub created_at: Option<DateTime<Utc>>,
}