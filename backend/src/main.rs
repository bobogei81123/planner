use anyhow::Context;
use dotenv::dotenv;
use futures::FutureExt;
use planner_backend::schedule_all_recurring_tasks_until;
use sea_orm::{Database, DatabaseConnection};
use tokio_cron_scheduler::{Job, JobScheduler};

use tracing::{error, info};
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

async fn setup_cron(db: DatabaseConnection) -> anyhow::Result<JobScheduler> {
    let sched = JobScheduler::new().await?;
    let cron_pattern = std::env::var("SCHEDULE_JOBS_CRON")
        .expect("The Cron pattern `SCHEDULE_JOBS_CRON` of the periodic schedule job must be set");
    info!(cron_pattern);
    let job = Job::new_async(cron_pattern, move |_, _| {
        let db = db.clone();
        async move {
            let result = schedule_all_recurring_tasks_until(&db, None).await;
            if let Err(err) = result {
                error!("Error when scheduling recurring tasks: {err:?}");
            }
        }
        .boxed()
    })?;
    job.on_start_notification_add(
        &sched,
        Box::new(|job_id, _, _| {
            async move {
                info!("Job {:?} started", job_id);
            }
            .boxed()
        }),
    )
    .await?;
    sched.add(job).await?;

    Ok(sched)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let db = connect_db().await;
    let app = planner_backend::build_app(db.clone()).await;
    let listener = tokio::net::TcpListener::bind(
        &std::env::var("BIND_ADDR")
            .expect("Server bind address $BIND_ADDR env variable is not set"),
    )
    .await
    .unwrap();

    let scheduler = setup_cron(db)
        .await
        .context("Failed to set up cron job")
        .unwrap();
    scheduler
        .start()
        .await
        .context("Failed to start scheduler")
        .unwrap();
    info!("Scheduler started");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Cannot create server")
}
