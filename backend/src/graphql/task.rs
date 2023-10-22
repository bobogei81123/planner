use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use super::{Iteration, IterationId, PgLoader, Result};

#[derive(Clone, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Task {
    id: Uuid,
    title: String,
    status: TaskStatus,
    point: Option<i32>,
    #[graphql(skip)]
    iterations: Vec<Uuid>,
}

#[derive(Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "task_status")]
#[sqlx(rename_all = "lowercase")]
#[derive(async_graphql::Enum)]
pub(crate) enum TaskStatus {
    Active,
    Completed,
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub(crate) struct TaskId(pub Uuid);

#[derive(sqlx::FromRow)]
pub struct PgTask {
    id: Uuid,
    title: String,
    status: TaskStatus,
    point: Option<i32>,
    iterations: Option<Vec<Uuid>>,
}

impl From<PgTask> for Task {
    fn from(
        PgTask {
            id,
            title,
            status,
            point,
            iterations,
        }: PgTask,
    ) -> Self {
        Task {
            id,
            title,
            status,
            point,
            iterations: iterations.unwrap_or_else(|| vec![]),
        }
    }
}

#[async_trait]
impl Loader<TaskId> for PgLoader {
    type Value = Task;
    type Error = Arc<sqlx::Error>;

    async fn load(
        &self,
        keys: &[TaskId],
    ) -> std::result::Result<HashMap<TaskId, Task>, Self::Error> {
        self.load_tasks(keys).await.map_err(Arc::new)
    }
}

impl PgLoader {
    pub(crate) async fn load_tasks(&self, keys: &[TaskId]) -> sqlx::Result<HashMap<TaskId, Task>> {
        let tasks: Vec<PgTask> = sqlx::query_as(
            r#"SELECT tasks.id, tasks.title, tasks.status, tasks.point,
                   array_remove(array_agg(iterations_tasks.iteration_id), NULL) AS iterations
               FROM tasks
               INNER JOIN users ON tasks.user_id = users.id
               LEFT JOIN iterations_tasks ON tasks.id = iterations_tasks.task_id
               WHERE users.username = $1 AND tasks.id = ANY($2)
               GROUP BY tasks.id;
            "#,
        )
        .bind("meteor")
        .bind(keys)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks
            .into_iter()
            .map(|x| (TaskId(x.id), x.into()))
            .collect())
    }
}

#[async_graphql::ComplexObject]
impl Task {
    async fn iterations(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Iteration>> {
        let loader = ctx.data_unchecked::<DataLoader<PgLoader>>();
        let iterations = loader
            .load_many(self.iterations.iter().map(|x| IterationId(*x)))
            .await?;

        Ok(iterations.into_values().collect())
    }
}

pub(crate) async fn get_all_tasks(ctx: &async_graphql::Context<'_>) -> Result<Vec<Task>> {
    let tasks: Vec<PgTask> = sqlx::query_as(
        r#"SELECT tasks.id, tasks.title, tasks.status, tasks.point,
               array_remove(array_agg(iterations_tasks.iteration_id), NULL) AS iterations
           FROM tasks
           INNER JOIN users ON tasks.user_id = users.id
           LEFT JOIN iterations_tasks ON tasks.id = iterations_tasks.task_id
           WHERE users.username = $1
           GROUP BY tasks.id;"#,
    )
    .bind("meteor")
    .fetch_all(ctx.data_unchecked::<PgPool>())
    .await?;

    Ok(tasks.into_iter().map(Task::from).collect())
}

#[cfg(test)]
mod tests {
    use testcontainers::clients::Cli;
    use testcontainers_modules::postgres::Postgres;

    use super::*;

    #[tokio::test]
    async fn sfd_test() -> Result<()> {
        let docker = Cli::default();
        let node = docker.run(Postgres::default());
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );
        let conn = PgPool::connect(&connection_string)
            .await?;
        let schema = include_str!("../../schema.sql");
        for result in sqlx::query(schema).execute_many(&conn) {
        }


        Ok(())
    }
}
