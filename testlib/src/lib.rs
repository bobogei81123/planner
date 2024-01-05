use std::sync::OnceLock;




use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use testcontainers::{clients::Cli, Container};
use testcontainers_modules::postgres::Postgres;

use uuid::Uuid;

pub type Result<T> = anyhow::Result<T>;

static DOCKER: OnceLock<Cli> = OnceLock::new();

pub struct PgDocker {
    #[allow(dead_code)]
    node: Container<'static, Postgres>,
    db_conn: DatabaseConnection,
}

impl PgDocker {
    pub async fn new() -> Self {
        let docker = DOCKER.get_or_init(Cli::default);
        let node = docker.run(Postgres::default());
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );
        let db_conn = Database::connect(connection_string)
            .await
            .expect("Failed to connect to docker postgres database");

        let schema = include_str!("../../backend/schema.sql");
        db_conn.execute_unprepared(schema).await.expect(
            "Failed to create initial database tables. \
                 Check schema.sql and see if there are any errors",
        );

        Self { node, db_conn }
    }

    pub fn db_conn(&self) -> &DatabaseConnection {
        &self.db_conn
    }
}

pub const fn test_uuid(b: u32) -> Uuid {
    Uuid::from_u128(b as u128)
}
