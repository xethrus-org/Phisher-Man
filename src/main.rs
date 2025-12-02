mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;

use axum::{
    routing::{get, post, delete, patch},
    Router,
};
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use db::create_pool;
use services::EmailService;

#[tokio::main]
async fn main() {
    // setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "phisherman=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // load config
    let config = Config::from_env();

    // create database pool
    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database pool created successfully");

    // create email service
    let email_service = EmailService::from_env();
    tracing::info!("Email service initialized");

    // build api routes
    let api_routes = Router::new()
        .route("/companies", post(handlers::create_company))
        .route("/companies", get(handlers::list_companies))
        .route("/companies/:id", get(handlers::get_company))
        .route("/companies/:id", patch(handlers::update_company))
        .route("/companies/:id", delete(handlers::delete_company))
        .route("/employees", post(handlers::create_employee))
        .route("/employees", get(handlers::list_employees))
        .route("/employees/:id", get(handlers::get_employee))
        .route("/employees/:id", patch(handlers::update_employee))
        .route("/employees/:id", delete(handlers::delete_employee))
        .route("/campaigns", post(handlers::create_campaign))
        .route("/campaigns", get(handlers::list_campaigns))
        .route("/campaigns/:id", get(handlers::get_campaign))
        .route("/campaigns/:id", patch(handlers::update_campaign))
        .route("/campaigns/:id", delete(handlers::delete_campaign))
        .route("/templates", post(handlers::create_template))
        .route("/templates", get(handlers::list_templates))
        .route("/templates/:id", get(handlers::get_template))
        .route("/templates/:id", patch(handlers::update_template))
        .route("/templates/:id", delete(handlers::delete_template))
        .route("/campaigns/:id/send", post(handlers::send_campaign));

    // build tracking routes (not under /api to avoid CORS issues with email clients)
    let tracking_routes = Router::new()
        .route("/track/:token", get(handlers::track_pixel))
        .route("/click/:token/:link_id", get(handlers::track_link));

    // build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_routes)
        .merge(tracking_routes)
        .nest_service("/", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state((pool, email_service));

    // run the server
    let listener = tokio::net::TcpListener::bind(&config.addr())
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server running on http://{}", config.addr());

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn health_check() -> &'static str {
    "ok"
}
