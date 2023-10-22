use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Bound, RangeBounds},
    sync::Arc,
};

use anyhow::{anyhow, bail};
use async_graphql::{
    dataloader::{DataLoader, Loader},
    http::GraphiQLSource,
    ComplexObject, Context, EmptySubscription, ErrorExtensions, Object, Schema, ID,
};
use async_graphql_axum::GraphQL;
use async_trait::async_trait;
use axum::{
    response::{Html, IntoResponse},
    routing, Router,
};
use chrono::NaiveDate;
use sqlx::{postgres::types::PgRange, Execute, PgPool, QueryBuilder};
use uuid::Uuid;

use self::task::{Task, TaskId, TaskStatus, get_all_tasks};

type Result<T> = std::result::Result<T, AppError>;

const METEOR_UUID: Uuid = uuid::uuid!("00000000-0000-4000-8001-000000000000");

pub struct QueryRoot;

pub struct PgLoader {
    pool: sqlx::PgPool,
}

mod task;
mod loader;

#[derive(Clone, sqlx::FromRow, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Iteration {
    id: Uuid,
    name: String,
    #[graphql(skip)]
    date_range: Option<PgRange<NaiveDate>>,
    #[graphql(skip)]
    tasks: Vec<Uuid>,
}

#[ComplexObject]
impl Iteration {
    async fn start_date(&self) -> Result<Option<NaiveDate>> {
        let Some(date_range) = &self.date_range else {
            return Ok(None);
        };
        match date_range.start_bound() {
            Bound::Included(x) => Ok(Some(*x)),
            other => Err(anyhow!(
                "Invalid start date: Expected it to be an included bound. Found {:#?}",
                other
            ))?,
        }
    }
    async fn end_date(&self) -> Result<Option<NaiveDate>> {
        let Some(date_range) = &self.date_range else {
            return Ok(None);
        };
        match date_range.end_bound() {
            Bound::Included(x) => Ok(Some(*x)),
            other => Err(anyhow!(
                "Invalid start date: Expected it to be an included bound. Found {:#?}",
                other
            ))?,
        }
    }

    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let loader = ctx.data_unchecked::<DataLoader<PgLoader>>();
        let tasks = loader
            .load_many(self.tasks.iter().map(|x| TaskId(*x)))
            .await?;

        Ok(tasks.into_values().collect())
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
struct IterationId(Uuid);

impl PgLoader {
    async fn load_iterations(
        &self,
        keys: &[IterationId],
    ) -> sqlx::Result<HashMap<IterationId, Iteration>> {
        let tasks: Vec<Iteration> = sqlx::query_as!(
            Iteration,
            r#"SELECT iterations.id, iterations.name, iterations.date_range,
                   array_remove(array_agg(iterations_tasks.task_id), NULL) AS "tasks!"
               FROM iterations
               INNER JOIN users ON iterations.user_id = users.id
               LEFT JOIN iterations_tasks ON iterations.id = iterations_tasks.iteration_id
               WHERE users.username = $1 AND iterations.id = ANY($2)
               GROUP BY iterations.id;
            "#,
            "meteor",
            keys as &[IterationId],
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks.into_iter().map(|x| (IterationId(x.id), x)).collect())
    }
}

#[async_trait]
impl Loader<IterationId> for PgLoader {
    type Value = Iteration;
    type Error = Arc<sqlx::Error>;

    async fn load(
        &self,
        keys: &[IterationId],
    ) -> std::result::Result<HashMap<IterationId, Iteration>, Self::Error> {
        self.load_iterations(keys).await.map_err(Arc::new)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("The resource with id = {0} is not found")]
    ResourceNotFound(Uuid),
    #[error("The request is invalid: {0}")]
    BadRequest(BadRequestReason),
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub enum BadRequestReason {
    InvalidDateRange,
}

impl Display for BadRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadRequestReason::InvalidDateRange => write!(
                f,
                "The date range is not valid. \
                 Start date or end date must be given if the other is, \
                 and the end date must be later than the start date."
            ),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Internal(value.into())
    }
}

impl From<Arc<sqlx::Error>> for AppError {
    fn from(value: Arc<sqlx::Error>) -> Self {
        AppError::Internal(value.into())
    }
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string()).extend_with(|_, e| {
            use AppError::*;
            match self {
                ResourceNotFound(..) => e.set("code", "NOT_FOUND"),
                BadRequest(..) => e.set("code", "BAD_REQUEST"),
                Internal(..) => {
                    e.set("code", "INTERNAL_SERVER_ERROR");
                }
            }
        })
    }
}

#[Object]
impl QueryRoot {
    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        get_all_tasks(ctx).await
    }

    async fn iterations(&self, ctx: &Context<'_>) -> Result<Vec<Iteration>> {
        let iters = sqlx::query_as!(
            Iteration,
            r#"SELECT iterations.id, iterations.name, iterations.date_range,
                   array_remove(array_agg(iterations_tasks.task_id), NULL) AS "tasks!"
               FROM iterations
               INNER JOIN users ON iterations.user_id = users.id
               LEFT JOIN iterations_tasks ON iterations.id = iterations_tasks.iteration_id
               WHERE users.username = $1
               GROUP BY iterations.id;
            "#,
            "meteor",
        )
        .fetch_all(ctx.data_unchecked::<PgPool>())
        .await?
        .into_iter()
        .collect();

        Ok(iters)
    }

    async fn iteration(&self, ctx: &Context<'_>, id: Uuid) -> Result<Iteration> {
        let loader = ctx.data_unchecked::<DataLoader<PgLoader>>();
        let iteration = loader
            .load_one(IterationId(id))
            .await?
            .ok_or(AppError::ResourceNotFound(id))?;

        Ok(iteration)
    }
}

pub struct MutationRoot;

#[derive(async_graphql::InputObject)]
struct UpdateTaskInput {
    id: Uuid,
    title: Option<String>,
    status: Option<TaskStatus>,
    point: Option<Option<i32>>,
    iterations: Option<Vec<Uuid>>,
}

#[derive(async_graphql::InputObject)]
struct CreateTaskInput {
    title: String,
    iteration: Option<Uuid>,
}

#[derive(async_graphql::InputObject)]
struct CreateIterationInput {
    name: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

#[Object]
impl MutationRoot {
    async fn update_task(&self, ctx: &Context<'_>, input: UpdateTaskInput) -> Result<Task> {
        let id = input.id;

        let db_conn = ctx.data_unchecked::<PgPool>();
        let mut tx = db_conn.begin().await?;
        if let Some(mut query_builder) = build_update_task_query(&input) {
            let query = query_builder.build();
            if query.execute(db_conn).await?.rows_affected() != 1 {
                tx.rollback().await?;
                return Err(AppError::ResourceNotFound(id));
            }
        }

        if let Some(iteration) = &input.iterations {
            sqlx::query(r#"DELETE FROM iterations_tasks WHERE task_id = $1"#)
                .bind(id)
                .execute(tx.as_mut())
                .await?;

            let query = sqlx::query(
                r#"INSERT INTO iterations_tasks (iteration_id, task_id)
                   SELECT
                     iteration_id,
                     $1
                   FROM UNNEST($2::uuid[]) AS t(iteration_id);"#,
            )
            .bind(id)
            .bind(iteration)
            .execute(tx.as_mut())
            .await?;
        }
        tx.commit().await?;

        let task = ctx
            .data_unchecked::<DataLoader<PgLoader>>()
            .load_one(TaskId(id))
            .await?
            .ok_or_else(|| AppError::ResourceNotFound(id))?;

        Ok(task)
    }

    async fn create_task(&self, ctx: &Context<'_>, input: CreateTaskInput) -> Result<Task> {
        let db_conn = ctx.data_unchecked::<PgPool>();
        let mut tx = db_conn.begin().await?;

        let id = Uuid::new_v4();
        let result = sqlx::query(
            r#"INSERT INTO tasks (id, user_id, title, status)
               VALUES ($1, $2, $3, $4)"#,
        )
        .bind(id)
        .bind(METEOR_UUID)
        .bind(&input.title)
        .bind(TaskStatus::Active)
        .execute(tx.as_mut())
        .await;

        match result {
            Err(err) => {
                tx.rollback().await?;
                return Err(err)?;
            }
            Ok(result) if result.rows_affected() != 1 => {
                tx.rollback().await?;
                return Err(anyhow!("Failed to insert task"))?;
            }
            Ok(_) => (),
        }

        if let Some(iteration) = input.iteration {
            let result = sqlx::query(
                r#"INSERT INTO iterations_tasks (iteration_id, task_id)
               VALUES ($1, $2)"#,
            )
            .bind(iteration)
            .bind(id)
            .execute(tx.as_mut())
            .await;

            match dbg!(result) {
                Err(sqlx::Error::Database(db_err))
                    if db_err
                        .constraint()
                        .is_some_and(|x| x == "iterations_tasks_iteration_id_fkey") =>
                {
                    return Err(AppError::ResourceNotFound(iteration))?
                }
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err)?;
                }
                Ok(result) if result.rows_affected() != 1 => {
                    tx.rollback().await?;
                    return Err(anyhow!("Failed to insert task"))?;
                }
                Ok(_) => (),
            }
        }
        tx.commit().await?;

        let task = ctx
            .data_unchecked::<DataLoader<PgLoader>>()
            .load_one(TaskId(id))
            .await?
            .ok_or_else(|| AppError::ResourceNotFound(id))?;

        Ok(task)
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Uuid> {
        let db_conn = ctx.data_unchecked::<PgPool>();

        let row_affected = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(db_conn)
            .await?
            .rows_affected();
        if row_affected != 1 {
            return Err(AppError::ResourceNotFound(id));
        }

        Ok(id)
    }

    async fn create_iteration(
        &self,
        ctx: &Context<'_>,
        input: CreateIterationInput,
    ) -> Result<Iteration> {
        let date_range: Option<PgRange<NaiveDate>> = match (input.start_date, input.end_date) {
            (None, None) => None,
            (Some(start_date), Some(end_date)) => Some(PgRange {
                start: Bound::Included(start_date),
                end: Bound::Included(end_date),
            }),
            (Some(_), None) | (None, Some(_)) => {
                return Err(AppError::BadRequest(BadRequestReason::InvalidDateRange))?
            }
        };
        let name = input.name.unwrap_or_else(|| "New Iteration".to_string());

        let db_conn = ctx.data_unchecked::<PgPool>();
        let row_affected = sqlx::query(
            r#"INSERT INTO iterations (id, user_id, name, date_range)
               VALUES ($1, $2, $3, $4)"#,
        )
        .bind(Uuid::new_v4())
        .bind(METEOR_UUID)
        .bind(&name)
        .bind(&date_range)
        .execute(db_conn)
        .await?
        .rows_affected();

        if row_affected != 1 {
            return Err(anyhow!("Failed to insert a new iteration"))?;
        }

        Ok(Iteration {
            id: Uuid::new_v4(),
            name,
            date_range,
            tasks: vec![],
        })
    }
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn routes(pool: PgPool) -> Router {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(async_graphql::extensions::Logger)
        .data(pool.clone())
        .data(DataLoader::new(PgLoader { pool }, tokio::spawn))
        .finish();

    Router::new().route(
        "/",
        routing::get(graphiql).post_service(GraphQL::new(schema)),
    )
}

fn build_update_task_query(input: &UpdateTaskInput) -> Option<sqlx::QueryBuilder<sqlx::Postgres>> {
    let mut query_builder = QueryBuilder::new("UPDATE tasks SET ");
    let mut has_changed = false;
    {
        let mut query_builder = query_builder.separated(", ");
        if let Some(title) = &input.title {
            query_builder.push_unseparated("title = ").push_bind(title);
            has_changed = true;
        }
        if let Some(status) = input.status {
            query_builder
                .push_unseparated("status = ")
                .push_bind(status);
            has_changed = true;
        }
        if let Some(point) = input.point {
            query_builder.push_unseparated("point = ").push_bind(point);
            has_changed = true;
        }
    }
    if !has_changed {
        return None;
    }

    query_builder.push(" WHERE id = ").push_bind(input.id);
    Some(query_builder)
}
