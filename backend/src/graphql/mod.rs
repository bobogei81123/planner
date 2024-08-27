use std::fmt::Display;

use crate::{
    app::{maybe::Maybe, task::ViewType, time::EpochLike},
    auth::Claims,
    entities,
    utils::OptionExt as _,
};
use async_graphql::{
    http::GraphiQLSource, Context, EmptySubscription, ErrorExtensions, InputObject, MaybeUndefined,
    Object, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing, Router,
};
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::app;

pub fn routes(db_conn: DatabaseConnection) -> Router {
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(async_graphql::extensions::Logger)
        .data(db_conn.clone())
        .finish();
    let app_state = AppState { db_conn, schema };

    Router::new()
        .route("/", routing::get(graphiql).post(graphql_handler))
        .with_state(app_state)
}

pub(crate) type AppSchema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub(crate) struct QueryRoot;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Bad request: {0}")]
    BadRequest(BadRequestReason),
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl Error {
    fn invalid_date_range(date_range: DateRange) -> Self {
        Self::BadRequest(BadRequestReason::InvalidDateRange(date_range))
    }

    fn required_field_is_null(field: String) -> Self {
        Error::BadRequest(BadRequestReason::RequiredFieldIsNull { field })
    }
}

#[derive(Debug)]
enum BadRequestReason {
    InvalidDateRange(DateRange),
    RequiredFieldIsNull { field: String },
}

impl Display for BadRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDateRange(date_range) => {
                write!(f, "the date range {date_range:?} is not valid")
            }
            Self::RequiredFieldIsNull { field } => {
                write!(f, "field `{field}` is a required field, but set to null")
            }
        }
    }
}

#[derive(Debug, InputObject)]
struct DateRange {
    start: NaiveDate,
    end: NaiveDate,
}

impl TryFrom<DateRange> for app::time::DateRange {
    type Error = Error;

    fn try_from(value: DateRange) -> Result<Self, Self::Error> {
        if value.start > value.end {
            return Err(Error::invalid_date_range(value));
        }
        Ok(app::time::DateRange::new(value.start, value.end))
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "InputEpoch")]
struct Epoch {
    type_: EpochType,
    date: NaiveDate,
}

#[derive(Copy, Clone, Eq, PartialEq, async_graphql::Enum)]
enum EpochType {
    Date,
    Week,
}

impl From<Epoch> for app::time::Epoch {
    fn from(value: Epoch) -> Self {
        match value.type_ {
            EpochType::Date => app::time::Epoch::Date(value.date),
            EpochType::Week => app::time::Epoch::Week(app::time::Week::from_start_date(value.date)),
        }
    }
}

impl From<app::time::Epoch> for Epoch {
    fn from(value: app::time::Epoch) -> Self {
        match value {
            app::time::Epoch::Date(date) => Epoch {
                type_: EpochType::Date,
                date,
            },
            app::time::Epoch::Week(week) => Epoch {
                type_: EpochType::Week,
                date: week.start_date(),
            },
        }
    }
}

#[derive(InputObject)]
struct TaskFilter {
    view_filter: Option<ViewFilter>,
}

#[derive(InputObject)]
struct ViewFilter {
    type_: ViewType,
    epoch: Option<Epoch>,
}

impl From<TaskFilter> for app::task::TaskFilter {
    fn from(value: TaskFilter) -> Self {
        app::task::TaskFilter {
            view_filter: value.view_filter.map(app::task::ViewFilter::from),
        }
    }
}

impl From<ViewFilter> for app::task::ViewFilter {
    fn from(value: ViewFilter) -> Self {
        app::task::ViewFilter {
            view_type: value.type_,
            epoch: value.epoch.map(app::time::Epoch::from),
        }
    }
}

#[derive(SimpleObject)]
struct Task {
    id: Uuid,
    scheduled_on: Option<Epoch>,
    is_completed: bool,
    title: String,
    cost: Option<i32>,
}

impl From<app::task::Task> for Task {
    fn from(value: app::task::Task) -> Self {
        let is_completed = value.is_completed();
        Self {
            id: value.id,
            scheduled_on: value.scheduled_on.map(From::from),
            is_completed,
            title: value.title,
            cost: value.cost,
        }
    }
}

#[Object]
impl QueryRoot {
    async fn tasks(
        &self,
        ctx: &Context<'_>,
        filter: Option<TaskFilter>,
    ) -> async_graphql::Result<Vec<Task>> {
        Ok(app::task::list_tasks(
            ctx.user()?.id,
            filter.try_map(TryInto::try_into)?.unwrap_or_default(),
            ctx.db_conn(),
        )
        .await?
        .into_iter()
        .map(Task::from)
        .collect::<Vec<_>>())
    }
}

#[derive(InputObject)]
struct CreateTaskInput {
    scheduled_on: Option<Epoch>,
    title: String,
    cost: Option<i32>,
}

impl From<CreateTaskInput> for app::task::CreateTaskInput {
    fn from(value: CreateTaskInput) -> Self {
        app::task::CreateTaskInput {
            scheduled_on: value.scheduled_on.map(From::from),
            title: value.title,
            cost: value.cost,
        }
    }
}

#[derive(InputObject)]
struct UpdateTaskInput {
    id: Uuid,
    scheduled_on: MaybeUndefined<Epoch>,
    complete_date: MaybeUndefined<NaiveDate>,
    title: MaybeUndefined<String>,
    cost: MaybeUndefined<i32>,
}

impl TryFrom<UpdateTaskInput> for app::task::UpdateTaskInput {
    type Error = Error;

    fn try_from(value: UpdateTaskInput) -> Result<Self, Self::Error> {
        Ok(app::task::UpdateTaskInput {
            id: value.id,
            scheduled_on: into_maybe(value.scheduled_on.map_value(From::from)),
            complete_date: into_maybe(value.complete_date),
            title: into_maybe_nonnull(value.title)
                .ok_or_else(|| Error::required_field_is_null("title".to_owned()))?,
            cost: into_maybe(value.cost),
        })
    }
}

fn into_maybe<T>(value: MaybeUndefined<T>) -> Maybe<Option<T>> {
    match value {
        MaybeUndefined::Value(x) => Maybe::Some(Some(x)),
        MaybeUndefined::Null => Maybe::Some(None),
        MaybeUndefined::Undefined => Maybe::Undefined,
    }
}

fn into_maybe_nonnull<T>(value: MaybeUndefined<T>) -> Option<Maybe<T>> {
    match value {
        MaybeUndefined::Value(x) => Some(Maybe::Some(x)),
        MaybeUndefined::Null => None,
        MaybeUndefined::Undefined => Some(Maybe::Undefined),
    }
}

pub(crate) struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
    ) -> async_graphql::Result<Task> {
        Ok(
            app::task::create_task(ctx.user()?.id, input.into(), ctx.db_conn())
                .await?
                .into(),
        )
    }

    async fn update_task(
        &self,
        ctx: &Context<'_>,
        input: UpdateTaskInput,
    ) -> async_graphql::Result<Task> {
        Ok(
            app::task::update_task(ctx.user()?.id, input.try_into()?, ctx.db_conn())
                .await?
                .into(),
        )
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Uuid> {
        app::task::delete_task(ctx.user()?.id, id, ctx.db_conn()).await?;
        Ok(id)
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
        self.data::<User>().map_err(|_| UnauthorizedError.extend())
    }

    fn db_conn(&self) -> &DatabaseConnection {
        self.data_unchecked::<DatabaseConnection>()
    }
}

#[derive(Debug)]
struct UnauthorizedError;

impl Display for UnauthorizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User is not authorized")
    }
}

impl std::error::Error for UnauthorizedError {}

impl ErrorExtensions for UnauthorizedError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string()).extend_with(|_, e| {
            e.set("code", "UNAUTHORIZED");
        })
    }
}
