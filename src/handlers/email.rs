use axum::{extract::{Path, State}, Json};
use regex::Regex;
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

        // First, insert the sent_email record to get the tracking token
        let sent_email = sqlx::query_as::<_, (Uuid, Uuid)>(
            "INSERT INTO sent_emails (campaign_id, employee_id, template_id, subject, body)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, tracking_token"
        )
        .bind(campaign_id)
        .bind(employee.id)
        .bind(payload.template_id)
        .bind(&template.subject)
        .bind(&template.body)
        .fetch_one(&pool)
        .await;

        if let Ok((sent_id, tracking_token)) = sent_email {
            // Replace links with tracking URLs
            let body_with_links = replace_links_with_tracking(
                &template.body,
                sent_id,
                tracking_token,
                &pool,
            ).await.unwrap_or_else(|_| template.body.clone());

            // Add tracking pixel to email body
            let tracking_pixel = format!(
                r#"<img src="http://localhost:3000/track/{}" width="1" height="1" alt="" style="display:none;" />"#,
                tracking_token
            );
            let body_with_tracking = format!("{}{}", body_with_links, tracking_pixel);

            match email_service.send_email(
                &employee.email,
                full_name.as_deref(),
                &template.subject,
                &body_with_tracking,
            ) {
                Ok(_) => {
                    sent_count += 1;
                }
                Err(e) => {
                    tracing::error!("Failed to send email to {}: {}", employee.email, e);
                    failed_count += 1;

                    // Delete the sent_email record since we failed to send
                    let _ = sqlx::query("DELETE FROM sent_emails WHERE id = $1")
                        .bind(sent_id)
                        .execute(&pool)
                        .await;
                }
            }
        } else {
            tracing::error!("Failed to create sent_email record for {}", employee.email);
            failed_count += 1;
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

async fn replace_links_with_tracking(
    html_body: &str,
    sent_email_id: Uuid,
    tracking_token: Uuid,
    pool: &PgPool,
) -> Result<String> {
    let re = Regex::new(r#"<a\s+(?:[^>]*\s+)?href=["']([^"']+)["']([^>]*)>"#).unwrap();
    let mut link_index = 0;
    let mut result = html_body.to_string();

    // Find all links and replace them
    let matches: Vec<_> = re.captures_iter(html_body).collect();

    for cap in matches.iter().rev() {
        let full_match = cap.get(0).unwrap();
        let original_url = cap.get(1).unwrap().as_str();
        let other_attrs = cap.get(2).unwrap().as_str();

        // Store the original URL in the database
        sqlx::query(
            "INSERT INTO tracked_links (sent_email_id, link_index, original_url) VALUES ($1, $2, $3)"
        )
        .bind(sent_email_id)
        .bind(link_index)
        .bind(original_url)
        .execute(pool)
        .await?;

        // Create tracking URL
        let tracking_url = format!("http://localhost:3000/click/{}/{}", tracking_token, link_index);
        let new_link = format!(r#"<a href="{}"{}>"#, tracking_url, other_attrs);

        // Replace the link
        let start = full_match.start();
        let end = full_match.end();
        result.replace_range(start..end, &new_link);

        link_index += 1;
    }

    Ok(result)
}
