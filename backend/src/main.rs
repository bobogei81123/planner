use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info,async-graphql=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn connect_db() -> DatabaseConnection {
    let postgres_conn_url = std::env::var("DATABASE_URL").expect("$DATABASE_URL is not set");

    Database::connect(postgres_conn_url)
        .await
        .expect("Cannot connect to Postgres")
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let app = planner_backend::build_app(connect_db().await).await;
    let listener = tokio::net::TcpListener::bind(
        &std::env::var("BIND_ADDR")
            .expect("Server bind address $BIND_ADDR env variable is not set"),
    )
    .await
    .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Cannot create server")
}
