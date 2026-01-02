use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{Date, DateTime, Utc};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Animal {
    pub id: i64,
    pub name: String,
    pub image: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
