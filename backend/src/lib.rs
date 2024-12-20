use axum::{Extension, Router};
use sea_orm::DatabaseConnection;

use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod app;
mod auth;
pub(crate) mod db;
pub mod entities;
mod graphql;
mod utils;
// mod batch_job;

// pub async fn build_app(pg_pool: PgPool) -> Router {
pub async fn build_app(pg_conn: DatabaseConnection) -> Router {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Router::new()
        .nest("/graphql", graphql::routes(pg_conn.clone()))
        .nest("/auth", auth::routes())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pg_conn))
}

pub use crate::app::task::schedule_all_recurring_tasks_until;
