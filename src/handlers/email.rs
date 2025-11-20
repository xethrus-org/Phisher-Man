use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Campaign, Employee};
use crate::services::EmailService;

#[derive(Debug, Deserialize)]
pub struct SendCampaignRequest {
    pub template_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SendCampaignResponse {
    pub sent_count: usize,
    pub failed_count: usize,
    pub message: String,
}

pub async fn send_campaign(
    State((pool, email_service)): State<(PgPool, EmailService)>,
    Path(campaign_id): Path<Uuid>,
    Json(payload): Json<SendCampaignRequest>,
) -> Result<Json<SendCampaignResponse>> {
    // Get campaign
    let campaign = sqlx::query_as::<_, Campaign>("SELECT * FROM campaigns WHERE id = $1")
        .bind(campaign_id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Campaign not found".to_string()))?;

    // Get template
    let template = sqlx::query_as::<_, crate::models::Template>(
        "SELECT * FROM templates WHERE id = $1"
    )
    .bind(payload.template_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Template not found".to_string()))?;

    // Get employees for this campaign's company
    let employees = sqlx::query_as::<_, Employee>(
        "SELECT * FROM employees WHERE company_id = $1"
    )
    .bind(campaign.company_id)
    .fetch_all(&pool)
    .await?;

    if employees.is_empty() {
        return Err(AppError::BadRequest("No employees found for this company".to_string()));
    }

    let mut sent_count = 0;
    let mut failed_count = 0;

    // Send email to each employee
    for employee in employees {
        let full_name = match (&employee.first_name, &employee.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            _ => None,
        };

        match email_service.send_email(
            &employee.email,
            full_name.as_deref(),
            &template.subject,
            &template.body,
        ) {
            Ok(_) => {
                sent_count += 1;

                // Record sent email in database
                let _ = sqlx::query(
                    r#"
                    INSERT INTO sent_emails (campaign_id, employee_id, template_id, subject, body)
                    VALUES ($1, $2, $3, $4, $5)
                    "#
                )
                .bind(campaign_id)
                .bind(employee.id)
                .bind(payload.template_id)
                .bind(&template.subject)
                .bind(&template.body)
                .execute(&pool)
                .await;
            }
            Err(e) => {
                tracing::error!("Failed to send email to {}: {}", employee.email, e);
                failed_count += 1;
            }
        }
    }

    // Update campaign status
    let _ = sqlx::query(
        r#"
        UPDATE campaigns
        SET status = 'active', started_at = NOW()
        WHERE id = $1
        "#
    )
    .bind(campaign_id)
    .execute(&pool)
    .await;

    Ok(Json(SendCampaignResponse {
        sent_count,
        failed_count,
        message: format!(
            "Campaign sent: {} successful, {} failed",
            sent_count, failed_count
        ),
    }))
}
