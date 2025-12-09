use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Company, CreateCompany, UpdateCompany};

pub async fn create_company(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Json(payload): Json<CreateCompany>,
) -> Result<(StatusCode, Json<Company>)> {
    let company = sqlx::query_as::<_, Company>(
        r#"
        INSERT INTO companies (name, domain)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.domain)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(company)))
}

pub async fn list_companies(State((pool, _)): State<(PgPool, crate::services::EmailService)>) -> Result<Json<Vec<Company>>> {
    let companies = sqlx::query_as::<_, Company>("SELECT * FROM companies ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;

    Ok(Json(companies))
}

pub async fn get_company(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Path(id): Path<Uuid>,
) -> Result<Json<Company>> {
    let company = sqlx::query_as::<_, Company>("SELECT * FROM companies WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Company not found".to_string()))?;

    Ok(Json(company))
}

pub async fn update_company(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCompany>,
) -> Result<Json<Company>> {
    // fetch existing company
    let mut company = sqlx::query_as::<_, Company>("SELECT * FROM companies WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Company not found".to_string()))?;

    // update fields if provided
    if let Some(name) = payload.name {
        company.name = name;
    }
    if let Some(domain) = payload.domain {
        company.domain = domain;
    }
    if let Some(settings) = payload.settings {
        company.settings = settings;
    }

    // save to database
    let updated = sqlx::query_as::<_, Company>(
        r#"
        UPDATE companies
        SET name = $1, domain = $2, settings = $3
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(&company.name)
    .bind(&company.domain)
    .bind(&company.settings)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}

pub async fn delete_company(State((pool, _)): State<(PgPool, crate::services::EmailService)>, Path(id): Path<Uuid>) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM companies WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Company not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
