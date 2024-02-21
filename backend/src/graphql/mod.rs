use std::{fmt::Display, sync::Arc, time::Duration};

use async_graphql::{
    dataloader::DataLoader, http::GraphiQLSource, Context, EmptySubscription, ErrorExtensions,
    Json, MaybeUndefined, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing, Router,
};
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{auth::Claims, entities};

use self::{
    iteration::{list_iterations, Iteration, IterationId},
    task::{create_task, delete_task, list_tasks, update_task, Task, TaskId, TaskStatus},
    task_schedule::{
        create_task_schedule, list_task_schedules, random_task_schedule, DateSpec, TaskSchedule,
    },
};

mod iteration;
mod loader;
mod task;
pub(crate) mod task_schedule;

pub fn routes(db_conn: DatabaseConnection) -> Router {
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(async_graphql::extensions::Logger)
        .data(db_conn.clone())
        .data(DataLoader::new(
            PgLoader {
                db_conn: db_conn.clone(),
            },
            tokio::spawn,
        ))
        .finish();
    let app_state = AppState { db_conn, schema };

    Router::new()
        .route("/", routing::get(graphiql).post(graphql_handler))
        .with_state(app_state)
}

pub(crate) type AppSchema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn tasks(
        &self,
        ctx: &Context<'_>,
        filter: Option<TaskFilter>,
    ) -> async_graphql::Result<Vec<Task>> {
        list_tasks(ctx.user()?.id, filter.unwrap_or_default(), ctx.db_conn())
            .await
            .map_err(|e| e.extend())
    }

    async fn iterations(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Iteration>> {
        list_iterations(ctx.user()?.id, ctx.db_conn())
            .await
            .map_err(|e| e.extend())
    }

    async fn iteration(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Iteration> {
        let loader = ctx.data_unchecked::<DataLoader<PgLoader>>();
        let iteration = loader
            .load_one(IterationId(id))
            .await?
            .ok_or(AppError::ResourceNotFound(id))?;

        Ok(iteration)
    }

    async fn task_schedules(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<TaskSchedule>> {
        // Ok(vec![random_task_schedule()])
        list_task_schedules(ctx.user()?.id, ctx.db_conn())
            .await
            .map_err(|e| e.extend())
    }
}

#[derive(Default, async_graphql::InputObject)]
struct TaskFilter {
    planned_date_range: Option<DateRange>,
}

#[derive(async_graphql::InputObject)]
struct DateRange {
    start: NaiveDate,
    end: NaiveDate,
}

pub(crate) struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn update_task(
        &self,
        ctx: &Context<'_>,
        input: UpdateTaskInput,
    ) -> async_graphql::Result<Task> {
        Ok(update_task(ctx.user()?.id, input, ctx.db_conn()).await?)
    }

    async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
    ) -> async_graphql::Result<Task> {
        Ok(create_task(ctx.user()?.id, input, ctx.db_conn()).await?)
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: TaskId) -> async_graphql::Result<TaskId> {
        delete_task(ctx.user()?.id, id, ctx.db_conn()).await?;
        Ok(id)
    }

    async fn create_iteration(
        &self,
        _ctx: &Context<'_>,
        _input: CreateIterationInput,
    ) -> async_graphql::Result<Iteration> {
        todo!("Decide if we should implement iteration feature at all")
        // let date_range: Option<PgRange<NaiveDate>> = match (input.start_date, input.end_date) {
        //     (None, None) => None,
        //     (Some(start_date), Some(end_date)) => Some(PgRange {
        //         start: Bound::Included(start_date),
        //         end: Bound::Included(end_date),
        //     }),
        //     (Some(_), None) | (None, Some(_)) => {
        //         return Err(AppError::BadRequest(BadRequestReason::InvalidDateRange))?
        //     }
        // };
        // let name = input.name.unwrap_or_else(|| "New Iteration".to_string());
        //
        // let id = Uuid::new_v4();
        // let db_conn = ctx.data_unchecked::<PgPool>();
        // let row_affected = sqlx::query(
        //     r#"INSERT INTO iterations (id, user_id, name, date_range)
        //        VALUES ($1, $2, $3, $4)"#,
        // )
        // .bind(id)
        // .bind(Uuid::new_v4())
        // .bind(&name)
        // .bind(&date_range)
        // .execute(db_conn)
        // .await?
        // .rows_affected();
        //
        // if row_affected != 1 {
        //     return Err(anyhow!("Failed to insert a new iteration"))?;
        // }
        //
        // let iteration = ctx
        //     .data_unchecked::<DataLoader<PgLoader>>()
        //     .load_one(IterationId(id))
        //     .await?
        //     .ok_or_else(|| AppError::ResourceNotFound(id))?;
        //
        // Ok(iteration)
    }

    async fn create_task_schedule(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskScheduleInput,
    ) -> async_graphql::Result<TaskSchedule> {
        Ok(create_task_schedule(ctx.user()?.id, input, ctx.db_conn()).await?)
    }
}

#[derive(async_graphql::InputObject, Default)]
struct UpdateTaskInput {
    id: Uuid,
    title: MaybeUndefined<String>,
    status: MaybeUndefined<TaskStatus>,
    point: MaybeUndefined<i32>,
    iterations: MaybeUndefined<Vec<Uuid>>,
    planned_on: MaybeUndefined<NaiveDate>,
}

#[derive(async_graphql::InputObject)]
struct CreateTaskInput {
    title: String,
    iteration: Option<Uuid>,
    point: Option<i32>,
    planned_on: Option<NaiveDate>,
}

#[derive(async_graphql::InputObject)]
struct CreateIterationInput {
    name: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

#[derive(async_graphql::InputObject)]
struct CreateTaskScheduleInput {
    date_spec: Json<DateSpec>,
    task_title: String,
    task_point: Option<i32>,
}

pub(crate) type AppResult<T> = Result<T, AppError>;
type DbResult<T> = Result<T, DbErr>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
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
pub(crate) enum BadRequestReason {
    InvalidDateRange,
    InvalidTaskScheduleSpec,
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
            BadRequestReason::InvalidTaskScheduleSpec => {
                write!(f, "The task schedule date spec JSON is not valid.")
            }
        }
    }
}

impl From<DbErr> for AppError {
    fn from(value: sea_orm::DbErr) -> Self {
        AppError::Internal(value.into())
    }
}

impl From<Arc<DbErr>> for AppError {
    fn from(value: Arc<sea_orm::DbErr>) -> Self {
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

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    State(AppState { db_conn, schema }): State<AppState>,
    claim: Option<Claims>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = request.into_inner();
    if let Some(claim) = claim {
        if let Ok(Some(user)) = claim.get_user(&db_conn).await {
            request = request.data(user);
        }
    }
    schema.execute(request).await.into()
}

#[derive(Clone)]
struct AppState {
    db_conn: DatabaseConnection,
    schema: AppSchema,
}

#[derive(Debug)]
struct User {
    id: Uuid,
    #[allow(dead_code)]
    username: String,
}

impl Claims {
    async fn get_user(&self, db_conn: &DatabaseConnection) -> anyhow::Result<Option<User>> {
        let username = &self.sub;

        Ok(entities::users::Entity::find()
            .filter(entities::users::Column::Username.eq(username))
            .one(db_conn)
            .await?
            .map(|user| User {
                id: user.id,
                username: user.username,
            }))
    }
}

#[extend::ext]
impl Context<'_> {
    fn user(&self) -> async_graphql::Result<&User> {
        self.data::<User>()
            .map_err(|_| AppError::Unauthorized.extend())
    }

    fn db_conn(&self) -> &DatabaseConnection {
        self.data_unchecked::<DatabaseConnection>()
    }
}

// TODO: Check user authorization when loading data
struct PgLoader {
    db_conn: DatabaseConnection,
}
