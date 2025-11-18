use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{CreateTemplate, Template, UpdateTemplate};

pub async fn create_template(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTemplate>,
) -> Result<(StatusCode, Json<Template>)> {
    let template = sqlx::query_as::<_, Template>(
        r#"
        INSERT INTO templates (name, subject, body, template_type, is_public, company_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.subject)
    .bind(&payload.body)
    .bind(&payload.template_type)
    .bind(&payload.is_public.unwrap_or(false))
    .bind(&payload.company_id)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(template)))
}

pub async fn list_templates(State(pool): State<PgPool>) -> Result<Json<Vec<Template>>> {
    let templates = sqlx::query_as::<_, Template>(
        "SELECT * FROM templates WHERE is_public = true OR company_id IS NOT NULL ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(templates))
}

pub async fn get_template(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Template>> {
    let template = sqlx::query_as::<_, Template>("SELECT * FROM templates WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Template not found".to_string()))?;

    Ok(Json(template))
}

pub async fn update_template(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTemplate>,
) -> Result<Json<Template>> {
    let mut template = sqlx::query_as::<_, Template>("SELECT * FROM templates WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Template not found".to_string()))?;

    if let Some(name) = payload.name {
        template.name = name;
    }
    if let Some(subject) = payload.subject {
        template.subject = subject;
    }
    if let Some(body) = payload.body {
        template.body = body;
    }
    if let Some(template_type) = payload.template_type {
        template.template_type = Some(template_type);
    }

    let updated = sqlx::query_as::<_, Template>(
        r#"
        UPDATE templates
        SET name = $1, subject = $2, body = $3, template_type = $4
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(&template.name)
    .bind(&template.subject)
    .bind(&template.body)
    .bind(&template.template_type)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}

pub async fn delete_template(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM templates WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Template not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
