use anyhow::anyhow;
use async_graphql::{
    http::GraphiQLSource, Context, EmptySubscription, ErrorExtensions, Object, Schema, ID,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing, Router,
};
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::model::{self, TaskStatus};

const METEOR_UUID: Uuid = uuid::uuid!("00000000-0000-4000-8001-000000000000");

pub struct QueryRoot;

#[derive(sqlx::FromRow, async_graphql::SimpleObject)]
pub struct Task {
    id: Uuid,
    title: String,
    status: model::TaskStatus,
    point: Option<i32>,
}

#[derive(async_graphql::SimpleObject)]
pub struct Iteration {
    id: Uuid,
    name: String,
    tasks: Vec<Task>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("The resource with id = {0} is not found")]
    ResourceNotFound(Uuid),
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Internal(value.into())
    }
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string()).extend_with(|_, e| {
            use AppError::*;
            match self {
                ResourceNotFound(..) => e.set("code", "NOT_FOUND"),
                Internal(..) => {
                    e.set("code", "INTERNAL_SERVER_ERROR");
                }
            }
        })
    }
}

#[Object]
impl QueryRoot {
    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>, AppError> {
        let tasks: Vec<Task> = sqlx::query_as(
            r#"SELECT tasks.id, tasks.title, tasks.status, tasks.point FROM tasks
               INNER JOIN users ON tasks.user_id = users.id
               WHERE users.username = $1"#,
        )
        .bind("meteor")
        .fetch_all(ctx.data_unchecked::<PgPool>())
        .await?;

        Ok(tasks)
    }

    async fn iterations(&self, ctx: &Context<'_>) -> Result<Vec<Uuid>, AppError> {
        let iters = sqlx::query!(
            r#"SELECT iterations.id FROM iterations
               INNER JOIN users ON iterations.user_id = users.id
               WHERE users.username = $1"#,
            "meteor"
        )
        .fetch_all(ctx.data_unchecked::<PgPool>())
        .await?
        .into_iter()
        .map(|row| row.id)
        .collect();

        Ok(iters)
    }

    async fn iteration(&self, ctx: &Context<'_>, id: Uuid) -> Result<Iteration, AppError> {
        let iter = sqlx::query!(
            r#"SELECT iterations.id, iterations.name FROM iterations
               INNER JOIN users ON iterations.user_id = users.id
               WHERE users.username = $1"#,
            "meteor"
        )
        .fetch_optional(ctx.data_unchecked::<PgPool>())
        .await?
        .ok_or_else(|| AppError::ResourceNotFound(id))?;

        let tasks: Vec<Task> = sqlx::query_as(
            r#"SELECT tasks.id, tasks.title, tasks.status, tasks.point FROM tasks
               INNER JOIN users ON tasks.user_id = users.id
               WHERE users.username = $1 AND tasks.planned_for = $2"#,
        )
        .bind("meteor")
        .bind(id)
        .fetch_all(ctx.data_unchecked::<PgPool>())
        .await?;

        Ok(Iteration {
            id: id,
            name: iter.name,
            tasks,
        })
    }
}

pub struct MutationRoot;

#[derive(async_graphql::InputObject)]
struct UpdateTaskInput {
    id: Uuid,
    title: Option<String>,
    status: Option<TaskStatus>,
    point: Option<Option<i32>>,
}

#[derive(async_graphql::InputObject)]
struct CreateTaskInput {
    title: String,
    planned_for: Option<Uuid>,
}

#[Object]
impl MutationRoot {
    async fn update_task(
        &self,
        ctx: &Context<'_>,
        input: UpdateTaskInput,
    ) -> Result<Task, AppError> {
        let db_conn = ctx.data_unchecked::<PgPool>();
        let id = input.id;

        if let Some(mut query_builder) = build_update_task_query(&input) {
            let query = query_builder.build();
            if query.execute(db_conn).await?.rows_affected() != 1 {
                return Err(AppError::ResourceNotFound(id));
            }
        }

        let task: Task = sqlx::query_as(
            r#"SELECT id, title, status, point FROM tasks
               WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(ctx.data_unchecked::<PgPool>())
        .await?
        .ok_or_else(|| AppError::ResourceNotFound(id))?;

        Ok(task)
    }

    async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
    ) -> Result<Task, AppError> {

        let db_conn = ctx.data_unchecked::<PgPool>();

        let id = Uuid::new_v4();

        let row_affected = sqlx::query(
            r#"INSERT INTO tasks (id, user_id, title, status, planned_for)
               VALUES ($1, $2, $3, 'active', $4)"#,
        )
        .bind(id)
        .bind(METEOR_UUID)
        .bind(&input.title)
        .bind(&input.planned_for)
        .execute(db_conn)
        .await?
        .rows_affected();
        if row_affected != 1 {
            Err(anyhow!("Failed to insert task"))?
        }

        Ok(Task {
            id,
            title: input.title,
            status: TaskStatus::Active,
            point: None,
        })
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Uuid, AppError> {
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
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn routes(pool: PgPool) -> Router {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(async_graphql::extensions::Logger)
        .data(pool)
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
