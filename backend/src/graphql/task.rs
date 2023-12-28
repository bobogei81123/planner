use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use sqlx::{postgres::types::PgRange, PgPool};
use uuid::Uuid;

use super::{
    iteration::{Iteration, IterationId},
    PgLoader, Result,
};

#[derive(Clone, Debug, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Task {
    id: Uuid,
    title: String,
    status: TaskStatus,
    point: Option<i32>,
    #[graphql(skip)]
    iterations: Vec<Uuid>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(type_name = "task_status")]
#[sqlx(rename_all = "lowercase")]
#[derive(async_graphql::Enum)]
pub enum TaskStatus {
    Active,
    Completed,
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub(crate) struct TaskId(pub Uuid);

// We need this struct because we cannot use `query_as!` macro and have to use the normal
// `query_as` function, which cannot assert the `iterations` field is not NULL.
#[derive(sqlx::FromRow)]
struct PgTask {
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
            iterations: iterations.unwrap_or_else(Vec::new),
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
        // We have to use `query_as` function instead of the macro because sqlx does not support
        // custom postgres enum type in macros.
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

pub(crate) async fn get_all_tasks(
    user_id: Uuid,
    ctx: &async_graphql::Context<'_>,
) -> Result<Vec<Task>> {
    let tasks: Vec<PgTask> = sqlx::query_as(
        r#"SELECT tasks.id, tasks.title, tasks.status, tasks.point,
               array_remove(array_agg(iterations_tasks.iteration_id), NULL) AS iterations
           FROM tasks
           LEFT JOIN iterations_tasks ON tasks.id = iterations_tasks.task_id
           WHERE tasks.user_id = $1
           GROUP BY tasks.id; "#,
    )
    .bind(user_id)
    .fetch_all(ctx.data_unchecked::<PgPool>())
    .await?;

    Ok(tasks.into_iter().map(Task::from).collect())
}

pub(crate) async fn get_planned_tasks_in_date_range(
    user_id: Uuid,
    date_range: (chrono::NaiveDate, chrono::NaiveDate),
    pool: &PgPool,
) -> Result<Vec<Task>> {
    let tasks: Vec<PgTask> = sqlx::query_as(
        r#"SELECT tasks.id, tasks.title, tasks.status, tasks.point,
               array_remove(array_agg(iterations_tasks.iteration_id), NULL) AS iterations
           FROM tasks
           LEFT JOIN iterations_tasks ON tasks.id = iterations_tasks.task_id
           WHERE tasks.user_id = $1
               AND $2 @> tasks.planned_on
           GROUP BY tasks.id; "#,
    )
    .bind(user_id)
    .bind(PgRange {
        start: std::ops::Bound::Included(date_range.0),
        end: std::ops::Bound::Excluded(date_range.1),
    })
    .fetch_all(pool)
    .await?;

    Ok(tasks.into_iter().map(Task::from).collect())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use googletest::prelude::*;

    use super::*;
    use testlib::{insert_task, test_uuid, PgDocker, Result, TestTaskBuilder};

    const DEFAULT_USER_UUID: Uuid = test_uuid(1);

    async fn insert_default_user(pg_docker: &PgDocker) {
        pg_docker
            .insert_test_user("meteor", DEFAULT_USER_UUID)
            .await
            .expect("failed to insert test user");
    }

    fn default_task_builder() -> TestTaskBuilder {
        let mut builder = TestTaskBuilder::default();
        builder.user_id(DEFAULT_USER_UUID);
        builder
    }

    #[googletest::test]
    #[tokio::test]
    async fn loader_can_load_tasks() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let pool = pg_docker.pool();
        let test_user_uuid = test_uuid(1);
        insert_default_user(&pg_docker).await;
        insert_task(
            pool,
            default_task_builder()
                .id(test_uuid(2))
                .title("test #1")
                .status(testlib::TaskStatus::Active)
                .point(Some(1))
                .build()
                .unwrap(),
        )
        .await;
        insert_task(
            pool,
            default_task_builder()
                .id(test_uuid(3))
                .title("test #2")
                .status(testlib::TaskStatus::Active)
                .point(Some(2))
                .build()
                .unwrap(),
        )
        .await;
        insert_task(
            pool,
            default_task_builder()
                .id(test_uuid(4))
                .title("test #3")
                .status(testlib::TaskStatus::Completed)
                .point(None)
                .build()
                .unwrap(),
        )
        .await;

        let loader = DataLoader::new(PgLoader { pool: pool.clone() }, tokio::spawn);
        let result = loader
            .load_many([TaskId(test_uuid(2)), TaskId(test_uuid(4))])
            .await?;

        expect_that!(
            result,
            unordered_elements_are![
                (
                    eq(TaskId(test_uuid(2))),
                    matches_pattern!(Task {
                        id: eq(test_uuid(2)),
                        title: eq("test #1"),
                        status: eq(TaskStatus::Active),
                        point: some(eq(1)),
                        iterations: empty(),
                    })
                ),
                (
                    eq(TaskId(test_uuid(4))),
                    matches_pattern!(Task {
                        id: eq(test_uuid(4)),
                        title: eq("test #3"),
                        status: eq(TaskStatus::Completed),
                        point: none(),
                        iterations: empty(),
                    })
                ),
            ]
        );

        Ok(())
    }

    #[googletest::test]
    #[tokio::test]
    async fn loader_can_load_tasks_planned_in_range() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let pool = pg_docker.pool();
        insert_default_user(&pg_docker).await;
        insert_task(
            pool,
            default_task_builder()
                .id(test_uuid(2))
                .title("test #1")
                .planned_on(Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()))
                .build()
                .unwrap(),
        )
        .await;
        insert_task(
            pool,
            default_task_builder()
                .id(test_uuid(3))
                .title("test #2")
                .planned_on(Some(NaiveDate::from_ymd_opt(2024, 1, 7).unwrap()))
                .build()
                .unwrap(),
        )
        .await;
        insert_task(
            pool,
            default_task_builder()
                .id(test_uuid(4))
                .title("test #3")
                .planned_on(Some(NaiveDate::from_ymd_opt(2024, 1, 5).unwrap()))
                .build()
                .unwrap(),
        )
        .await;

        let result = get_planned_tasks_in_date_range(
            DEFAULT_USER_UUID,
            (
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 6).unwrap(),
            ),
            &pool,
        )
        .await?;

        expect_that!(
            result,
            unordered_elements_are![
                matches_pattern!(Task {
                    id: eq(test_uuid(2)),
                    title: eq("test #1"),
                }),
                matches_pattern!(Task {
                    id: eq(test_uuid(4)),
                    title: eq("test #3"),
                }),
            ]
        );

        Ok(())
    }
}
