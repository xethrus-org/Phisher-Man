use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{CreateEmployee, Employee, UpdateEmployee};

#[derive(Debug, Deserialize)]
pub struct ListEmployeesQuery {
    pub company_id: Option<Uuid>,
}

pub async fn create_employee(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Json(payload): Json<CreateEmployee>,
) -> Result<(StatusCode, Json<Employee>)> {
    let employee = sqlx::query_as::<_, Employee>(
        r#"
        INSERT INTO employees (company_id, email, first_name, last_name, department, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&payload.company_id)
    .bind(&payload.email)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.department)
    .bind(&payload.metadata.unwrap_or(serde_json::json!({})))
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(employee)))
}

pub async fn list_employees(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Query(params): Query<ListEmployeesQuery>,
) -> Result<Json<Vec<Employee>>> {
    let employees = if let Some(company_id) = params.company_id {
        sqlx::query_as::<_, Employee>(
            "SELECT * FROM employees WHERE company_id = $1 ORDER BY created_at DESC",
        )
        .bind(company_id)
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as::<_, Employee>("SELECT * FROM employees ORDER BY created_at DESC")
            .fetch_all(&pool)
            .await?
    };

    Ok(Json(employees))
}

pub async fn get_employee(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Path(id): Path<Uuid>,
) -> Result<Json<Employee>> {
    let employee = sqlx::query_as::<_, Employee>("SELECT * FROM employees WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Employee not found".to_string()))?;

    Ok(Json(employee))
}

pub async fn update_employee(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateEmployee>,
) -> Result<Json<Employee>> {
    let mut employee = sqlx::query_as::<_, Employee>("SELECT * FROM employees WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Employee not found".to_string()))?;

    if let Some(email) = payload.email {
        employee.email = email;
    }
    if let Some(first_name) = payload.first_name {
        employee.first_name = Some(first_name);
    }
    if let Some(last_name) = payload.last_name {
        employee.last_name = Some(last_name);
    }
    if let Some(department) = payload.department {
        employee.department = Some(department);
    }
    if let Some(metadata) = payload.metadata {
        employee.metadata = metadata;
    }

    let updated = sqlx::query_as::<_, Employee>(
        r#"
        UPDATE employees
        SET email = $1, first_name = $2, last_name = $3, department = $4, metadata = $5
        WHERE id = $6
        RETURNING *
        "#,
    )
    .bind(&employee.email)
    .bind(&employee.first_name)
    .bind(&employee.last_name)
    .bind(&employee.department)
    .bind(&employee.metadata)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}

pub async fn delete_employee(
    State((pool, _)): State<(PgPool, crate::services::EmailService)>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM employees WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Employee not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
