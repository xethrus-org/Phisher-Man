use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::analytics::{CampaignAnalytics, DepartmentStats};

pub async fn get_campaign_analytics(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Path(campaign_id): Path<Uuid>,
) -> Result<Json<CampaignAnalytics>> {
    // Get campaign details
    let campaign = sqlx::query_as::<_, (String,)>(
        "SELECT name FROM campaigns WHERE id = $1"
    )
    .bind(campaign_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Campaign not found".to_string()))?;

    // Get overall campaign stats
    let stats = sqlx::query_as::<_, (i64, i64, i64)>(
        r#"
        SELECT
            COUNT(DISTINCT se.id) as total_sent,
            COUNT(DISTINCT CASE WHEN i.interaction_type = 'email_opened' THEN se.id END) as total_opened,
            COUNT(DISTINCT CASE WHEN i.interaction_type = 'link_clicked' THEN se.id END) as total_clicked
        FROM sent_emails se
        LEFT JOIN interactions i ON i.sent_email_id = se.id
        WHERE se.campaign_id = $1
        "#
    )
    .bind(campaign_id)
    .fetch_one(&pool)
    .await?;

    let (total_sent, total_opened, total_clicked) = stats;

    let open_rate = if total_sent > 0 {
        (total_opened as f64 / total_sent as f64) * 100.0
    } else {
        0.0
    };

    let click_rate = if total_sent > 0 {
        (total_clicked as f64 / total_sent as f64) * 100.0
    } else {
        0.0
    };

    // Get department breakdown
    let dept_stats = sqlx::query_as::<_, (Option<String>, i64, i64, i64, i64)>(
        r#"
        SELECT
            COALESCE(e.department, 'Unknown') as department,
            COUNT(DISTINCT e.id) as employee_count,
            COUNT(DISTINCT se.id) as emails_sent,
            COUNT(DISTINCT CASE WHEN i.interaction_type = 'email_opened' THEN se.id END) as emails_opened,
            COUNT(DISTINCT CASE WHEN i.interaction_type = 'link_clicked' THEN se.id END) as links_clicked
        FROM employees e
        LEFT JOIN sent_emails se ON se.employee_id = e.id AND se.campaign_id = $1
        LEFT JOIN interactions i ON i.sent_email_id = se.id
        WHERE e.company_id = (SELECT company_id FROM campaigns WHERE id = $1)
        GROUP BY e.department
        ORDER BY links_clicked DESC
        "#
    )
    .bind(campaign_id)
    .fetch_all(&pool)
    .await?;

    let departments: Vec<DepartmentStats> = dept_stats
        .into_iter()
        .map(|(dept, employee_count, emails_sent, emails_opened, links_clicked)| {
            let dept_name = dept.unwrap_or_else(|| "Unknown".to_string());
            let open_rate = if emails_sent > 0 {
                (emails_opened as f64 / emails_sent as f64) * 100.0
            } else {
                0.0
            };
            let click_rate = if emails_sent > 0 {
                (links_clicked as f64 / emails_sent as f64) * 100.0
            } else {
                0.0
            };

            DepartmentStats {
                department: dept_name,
                employee_count,
                emails_sent,
                emails_opened,
                links_clicked,
                open_rate,
                click_rate,
            }
        })
        .collect();

    Ok(Json(CampaignAnalytics {
        campaign_id: campaign_id.to_string(),
        campaign_name: campaign.0,
        total_sent,
        total_opened,
        total_clicked,
        open_rate,
        click_rate,
        departments,
    }))
}
