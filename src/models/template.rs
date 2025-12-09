use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub template_type: Option<String>,
    pub is_public: bool,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplate {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub template_type: Option<String>,
    pub is_public: Option<bool>,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplate {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub template_type: Option<String>,
}
