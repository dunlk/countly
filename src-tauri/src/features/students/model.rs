use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Student {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}
