use std::sync::OnceLock;

use chrono::NaiveDate;
use derive_builder::Builder;
use sqlx::{Executor, PgPool};
use testcontainers::{clients::Cli, Container};
use testcontainers_modules::postgres::Postgres;
use tokio_stream::StreamExt as _;
use uuid::Uuid;

pub type Result<T> = anyhow::Result<T>;

static DOCKER: OnceLock<Cli> = OnceLock::new();

pub struct PgDocker {
    #[allow(dead_code)]
    node: Container<'static, Postgres>,
    pool: PgPool,
}

impl PgDocker {
    pub async fn new() -> Self {
        let docker = DOCKER.get_or_init(Cli::default);
        let node = docker.run(Postgres::default());
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );
        let pool = PgPool::connect(connection_string)
            .await
            .expect("Failed to connect to docker postgres database");

        let schema = include_str!("../../backend/schema.sql");
        let mut result_stream = pool.execute_many(schema);
        while let Some(result) = result_stream.next().await {
            result.expect(
                "Failed to create initial database tables. \
                     Check schema.sql and see if there are any errors",
            );
        }

        Self { node, pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn insert_test_user(&self, username: &str, uuid: Uuid) -> Result<()> {
        sqlx::query("INSERT INTO users (id, username) VALUES ($1, $2)")
            .bind(uuid)
            .bind(username)
            .execute(self.pool())
            .await?;

        Ok(())
    }
}

pub const fn test_uuid(b: u32) -> Uuid {
    Uuid::from_u128(b as u128)
}

#[derive(Builder)]
pub struct TestTask {
    id: Uuid,
    user_id: Uuid,
    #[builder(default, setter(into))]
    title: String,
    #[builder(default = "TaskStatus::Active")]
    status: TaskStatus,
    #[builder(default)]
    point: Option<i32>,
    #[builder(default)]
    planned_on: Option<NaiveDate>,
}

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(type_name = "task_status")]
#[sqlx(rename_all = "lowercase")]
pub enum TaskStatus {
    Active,
    Completed,
}

pub async fn insert_task(
    pool: &PgPool,
    TestTask {
        id,
        user_id,
        title,
        status,
        point,
        planned_on,
    }: TestTask,
) {
    let title = &title;
    sqlx::query(
        r#"
        INSERT INTO tasks(id, user_id, title, status, point, planned_on)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(title)
    .bind(status)
    .bind(point)
    .bind(planned_on)
    .execute(pool)
    .await
    .unwrap_or_else(|e| {
        panic!(
            "Failed to insert task with id={id:#?}, user_id={user_id:#?}, \
                title={title:#?}, status={status:#?}, point={point:#?}. \
                This may be caused by invalid inputs: {e:?}"
        )
    });
}
