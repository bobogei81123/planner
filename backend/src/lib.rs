use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod auth;
mod graphql;
mod db;

pub async fn build_app(pg_pool: PgPool) -> Router {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Router::new()
        .nest("/graphql", graphql::routes(pg_pool.clone()))
        .nest("/auth", auth::routes())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pg_pool))
}
