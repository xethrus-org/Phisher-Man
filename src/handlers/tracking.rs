use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;

/// Tracking pixel endpoint - records email opens
/// Returns a 1x1 transparent PNG image
pub async fn track_pixel(
    State(pool): State<PgPool>,
    Path(tracking_token): Path<Uuid>,
) -> impl IntoResponse {
    // Record the interaction asynchronously (don't wait for result)
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let _ = record_interaction(
            &pool_clone,
            tracking_token,
            "email_opened"
        ).await;
    });

    // Return 1x1 transparent PNG
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
        0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00,
        0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00,
        0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/png")],
        png_data,
    )
}

/// Link click tracking endpoint
/// Records the click and redirects to the actual URL
pub async fn track_link(
    State(pool): State<PgPool>,
    Path((tracking_token, link_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    // Record the interaction asynchronously
    let pool_clone = pool.clone();
    let link_id_clone = link_id.clone();
    tokio::spawn(async move {
        let _ = record_interaction_with_metadata(
            &pool_clone,
            tracking_token,
            "link_clicked",
            serde_json::json!({ "link_id": link_id_clone })
        ).await;
    });

    // Look up the original URL from the database
    let link_index: i32 = link_id.parse().unwrap_or(0);

    let original_url = sqlx::query_as::<_, (String,)>(
        "SELECT tl.original_url
         FROM tracked_links tl
         JOIN sent_emails se ON tl.sent_email_id = se.id
         WHERE se.tracking_token = $1 AND tl.link_index = $2"
    )
    .bind(tracking_token)
    .bind(link_index)
    .fetch_optional(&pool)
    .await;

    match original_url {
        Ok(Some((url,))) => {
            // Redirect to the original URL
            (
                StatusCode::SEE_OTHER,
                [(header::LOCATION, url)],
            )
        }
        _ => {
            // Fallback to a default page if URL not found
            (
                StatusCode::SEE_OTHER,
                [(header::LOCATION, "/".to_string())],
            )
        }
    }
}

/// Helper function to record an interaction
async fn record_interaction(
    pool: &PgPool,
    tracking_token: Uuid,
    interaction_type: &str,
) -> Result<()> {
    // Get the sent_email_id from tracking_token
    let sent_email = sqlx::query_as::<_, (Uuid,)>(
        "SELECT id FROM sent_emails WHERE tracking_token = $1"
    )
    .bind(tracking_token)
    .fetch_optional(pool)
    .await?;

    if let Some((email_id,)) = sent_email {
        // Check if this interaction already exists (prevent duplicates for opens)
        if interaction_type == "email_opened" {
            let existing = sqlx::query_as::<_, (Uuid,)>(
                "SELECT id FROM interactions WHERE sent_email_id = $1 AND interaction_type = $2"
            )
            .bind(email_id)
            .bind(interaction_type)
            .fetch_optional(pool)
            .await?;

            if existing.is_some() {
                // Already recorded, skip
                return Ok(());
            }
        }

        // Insert interaction
        sqlx::query(
            "INSERT INTO interactions (sent_email_id, interaction_type, metadata) VALUES ($1, $2, '{}')"
        )
        .bind(email_id)
        .bind(interaction_type)
        .execute(pool)
        .await?;

        tracing::info!(
            "Recorded {} for sent_email_id: {}",
            interaction_type,
            email_id
        );
    } else {
        tracing::warn!("Tracking token not found: {}", tracking_token);
    }

    Ok(())
}

/// Helper function to record an interaction with metadata
async fn record_interaction_with_metadata(
    pool: &PgPool,
    tracking_token: Uuid,
    interaction_type: &str,
    metadata: serde_json::Value,
) -> Result<()> {
    // Get the sent_email_id from tracking_token
    let sent_email = sqlx::query_as::<_, (Uuid,)>(
        "SELECT id FROM sent_emails WHERE tracking_token = $1"
    )
    .bind(tracking_token)
    .fetch_optional(pool)
    .await?;

    if let Some((email_id,)) = sent_email {
        // Insert interaction with metadata
        sqlx::query(
            "INSERT INTO interactions (sent_email_id, interaction_type, metadata) VALUES ($1, $2, $3)"
        )
        .bind(email_id)
        .bind(interaction_type)
        .bind(metadata)
        .execute(pool)
        .await?;

        tracing::info!(
            "Recorded {} for sent_email_id: {}",
            interaction_type,
            email_id
        );
    } else {
        tracing::warn!("Tracking token not found: {}", tracking_token);
    }

    Ok(())
}
