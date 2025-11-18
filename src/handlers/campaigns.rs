use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Campaign, CreateCampaign, UpdateCampaign};

#[derive(Debug, Deserialize)]
pub struct ListCampaignsQuery {
    pub company_id: Option<Uuid>,
}

pub async fn create_campaign(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCampaign>,
) -> Result<(StatusCode, Json<Campaign>)> {
    let campaign = sqlx::query_as::<_, Campaign>(
        r#"
        INSERT INTO campaigns (company_id, name, description)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&payload.company_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(campaign)))
}

pub async fn list_campaigns(
    State(pool): State<PgPool>,
    Query(params): Query<ListCampaignsQuery>,
) -> Result<Json<Vec<Campaign>>> {
    let campaigns = if let Some(company_id) = params.company_id {
        sqlx::query_as::<_, Campaign>(
            "SELECT * FROM campaigns WHERE company_id = $1 ORDER BY created_at DESC",
        )
        .bind(company_id)
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as::<_, Campaign>("SELECT * FROM campaigns ORDER BY created_at DESC")
            .fetch_all(&pool)
            .await?
    };

    Ok(Json(campaigns))
}

pub async fn get_campaign(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Campaign>> {
    let campaign = sqlx::query_as::<_, Campaign>("SELECT * FROM campaigns WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Campaign not found".to_string()))?;

    Ok(Json(campaign))
}

pub async fn update_campaign(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCampaign>,
) -> Result<Json<Campaign>> {
    let mut campaign = sqlx::query_as::<_, Campaign>("SELECT * FROM campaigns WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Campaign not found".to_string()))?;

    if let Some(name) = payload.name {
        campaign.name = name;
    }
    if let Some(description) = payload.description {
        campaign.description = Some(description);
    }
    if let Some(status) = payload.status {
        campaign.status = status;
    }

    let updated = sqlx::query_as::<_, Campaign>(
        r#"
        UPDATE campaigns
        SET name = $1, description = $2, status = $3
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(&campaign.name)
    .bind(&campaign.description)
    .bind(&campaign.status)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}

pub async fn delete_campaign(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM campaigns WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Campaign not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
