use chrono::NaiveDate;
use clap::Parser;
use dotenv::dotenv;
use planner_backend::schedule_all_recurring_tasks_until;
use sea_orm::{Database, DatabaseConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(long)]
    until: Option<NaiveDate>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    dotenv().ok();
    let args = CliArgs::parse();
    let db_conn = connect_db().await;

    schedule_all_recurring_tasks_until(&db_conn, args.until).await?;
    Ok(())
}

async fn connect_db() -> DatabaseConnection {
    let postgres_conn_url = std::env::var("DATABASE_URL").expect("$DATABASE_URL is not set");

    Database::connect(postgres_conn_url)
        .await
        .expect("Cannot connect to Postgres")
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
