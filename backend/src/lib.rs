use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Extension, Router};
use sqlx::PgPool;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod graphql;
mod model;

pub async fn build_app(pg_pool: PgPool) -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Router::new()
        .nest("/graphql", graphql::routes(pg_pool.clone()))
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pg_pool))
}
