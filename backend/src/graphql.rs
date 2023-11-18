use std::{fmt::Display, ops::Bound, sync::Arc};

use anyhow::anyhow;
use async_graphql::{
    dataloader::DataLoader, http::GraphiQLSource, Context, EmptySubscription, ErrorExtensions,
    Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing, Router,
};
use chrono::NaiveDate;
use futures::FutureExt;
use sqlx::{postgres::types::PgRange, PgPool, QueryBuilder};
use uuid::Uuid;

use crate::{auth::Claims, db::TransactionExt};

use self::{
    iteration::{get_all_iterations, Iteration, IterationId},
    task::{get_all_tasks, Task, TaskId, TaskStatus},
};

type Result<T> = std::result::Result<T, AppError>;

pub struct QueryRoot;
pub struct MutationRoot;

pub struct PgLoader {
    pool: sqlx::PgPool,
}

mod iteration;
mod loader;
mod task;

type AppSchema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("The resource with id = {0} is not found")]
    ResourceNotFound(Uuid),
    #[error("The request is invalid: {0}")]
    BadRequest(BadRequestReason),
    #[error("User is not authorized")]
    Unauthorized,
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
                Unauthorized => e.set("code", "FORBIDDEN"),
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
        get_all_tasks(get_user_from_ctx(ctx)?.id, ctx).await
    }

    async fn iterations(&self, ctx: &Context<'_>) -> Result<Vec<Iteration>> {
        get_all_iterations(get_user_from_ctx(ctx)?.id, ctx).await
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
        let db_conn = ctx.data_unchecked::<PgPool>().clone();
        let tx = db_conn.begin().await?;

        tx.with(|mut tx| async move {
            if let Some(mut query_builder) = build_update_task_query(&input) {
                println!("query = {:?}", query_builder.sql());
                let query = query_builder.build();
                if query.execute(&db_conn).await?.rows_affected() != 1 {
                    return Err(AppError::ResourceNotFound(id));
                }
            }

            if let Some(iteration) = &input.iterations {
                sqlx::query(r#"DELETE FROM iterations_tasks WHERE task_id = $1;"#)
                    .bind(id)
                    .execute(tx.as_mut())
                    .await?;

                sqlx::query(
                    r#"
                        INSERT INTO iterations_tasks (iteration_id, task_id)
                        SELECT iteration_id, $1
                        FROM UNNEST($2::uuid[]) AS t(iteration_id);
                        "#,
                )
                .bind(id)
                .bind(iteration)
                .execute(tx.as_mut())
                .await?;
            }

            Ok(())
        })
        .await?;

        let task = ctx
            .data_unchecked::<DataLoader<PgLoader>>()
            .load_one(TaskId(id))
            .await?
            .ok_or_else(|| AppError::ResourceNotFound(id))?;

        Ok(task)
    }

    async fn create_task(&self, ctx: &Context<'_>, input: CreateTaskInput) -> Result<Task> {
        println!("user = ???");
        let user = get_user_from_ctx(ctx)?;
        println!("user = {:?}", user);
        let db_conn = ctx.data_unchecked::<PgPool>();
        let task_id = Uuid::new_v4();

        let tx = db_conn.begin().await?;

        tx.with(|mut tx| {
            async move {
                let result = sqlx::query(
                    r#"
                    INSERT INTO tasks (id, user_id, title, status)
                    VALUES ($1, $2, $3, $4);
                    "#,
                )
                .bind(task_id)
                .bind(user.id)
                .bind(&input.title)
                .bind(TaskStatus::Active)
                .execute(tx.as_mut())
                .await?;

                if result.rows_affected() != 1 {
                    return Err(anyhow!("Failed to insert task"))?;
                }

                let Some(iteration) = input.iteration else {
                    return Ok(());
                };

                let result = sqlx::query(
                    r#"
                    INSERT INTO iterations_tasks (iteration_id, task_id)
                    VALUES ($1, $2);
                    "#,
                )
                .bind(iteration)
                .bind(task_id)
                .execute(tx.as_mut())
                .await;

                match result {
                    Err(sqlx::Error::Database(db_err))
                        if db_err
                            .constraint()
                            .is_some_and(|x| x == "iterations_tasks_iteration_id_fkey") =>
                    {
                        Err(AppError::ResourceNotFound(iteration))?
                    }
                    Err(err) => Err(err)?,
                    Ok(result) if result.rows_affected() != 1 => {
                        Err(anyhow!("Failed to insert task"))?
                    }
                    Ok(_) => Ok::<_, AppError>(()),
                }
            }
            .boxed()
        })
        .await?;

        let task = ctx
            .data_unchecked::<DataLoader<PgLoader>>()
            .load_one(TaskId(task_id))
            .await?
            .ok_or_else(|| AppError::ResourceNotFound(task_id))?;

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

        let id = Uuid::new_v4();
        let db_conn = ctx.data_unchecked::<PgPool>();
        let row_affected = sqlx::query(
            r#"INSERT INTO iterations (id, user_id, name, date_range)
               VALUES ($1, $2, $3, $4)"#,
        )
        .bind(id)
        .bind(Uuid::new_v4())
        .bind(&name)
        .bind(&date_range)
        .execute(db_conn)
        .await?
        .rows_affected();

        if row_affected != 1 {
            return Err(anyhow!("Failed to insert a new iteration"))?;
        }

        let iteration = ctx
            .data_unchecked::<DataLoader<PgLoader>>()
            .load_one(IterationId(id))
            .await?
            .ok_or_else(|| AppError::ResourceNotFound(id))?;

        Ok(iteration)
    }
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    State(AppState { pool, schema }): State<AppState>,
    claim: Option<Claims>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = request.into_inner();
    if let Some(claim) = claim {
        if let Ok(Some(user)) = get_user_from_claim(claim, pool).await {
            request = request.data(user);
        }
    }
    schema.execute(request).await.into()
}

pub fn routes(pool: PgPool) -> Router {
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(async_graphql::extensions::Logger)
        .data(pool.clone())
        .data(DataLoader::new(
            PgLoader { pool: pool.clone() },
            tokio::spawn,
        ))
        .finish();
    let app_state = AppState { pool, schema };

    Router::new()
        .route("/", routing::get(graphiql).post(graphql_handler))
        .with_state(app_state)
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    schema: AppSchema,
}

fn build_update_task_query(input: &UpdateTaskInput) -> Option<sqlx::QueryBuilder<sqlx::Postgres>> {
    let mut query_builder = QueryBuilder::new("UPDATE tasks SET ");
    let mut has_changed = false;
    {
        let mut query_builder = query_builder.separated(", ");
        if let Some(title) = &input.title {
            query_builder.push("title = ").push_bind_unseparated(title);
            has_changed = true;
        }
        if let Some(status) = input.status {
            query_builder
                .push("status = ")
                .push_bind_unseparated(status);
            has_changed = true;
        }
        if let Some(point) = input.point {
            query_builder.push("point = ").push_bind_unseparated(point);
            has_changed = true;
        }
    }
    if !has_changed {
        return None;
    }

    query_builder.push(" WHERE id = ").push_bind(input.id);
    Some(query_builder)
}

#[derive(Debug)]
struct User {
    id: Uuid,
    username: String,
}

async fn get_user_from_claim(claim: Claims, pool: PgPool) -> anyhow::Result<Option<User>> {
    let username = claim.sub;

    Ok(
        sqlx::query_scalar!("SELECT id FROM users WHERE username = $1", username)
            .fetch_optional(&pool)
            .await?
            .map(|id| User { id, username }),
    )
}

fn get_user_from_ctx<'a>(ctx: &'a Context<'_>) -> Result<&'a User> {
    ctx.data::<User>().map_err(|_| AppError::Unauthorized)
}
