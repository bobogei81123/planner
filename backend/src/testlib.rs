use std::sync::OnceLock;

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

        let schema = include_str!("../schema.sql");
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
