use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub domain: String,
    pub created_at: DateTime<Utc>,
    pub settings: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CreateCompany {
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCompany {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub settings: Option<serde_json::Value>,
}
