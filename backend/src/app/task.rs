use anyhow::Context;
use async_graphql::MaybeUndefined;
use chrono::NaiveDate;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, NotSet},
    ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{
    app::time::EpochLike,
    entities::{self, task::Model as TaskModel},
};

use super::{maybe::Maybe, time::Epoch, AppError, AppResult};

#[derive(Clone, Debug)]
pub(crate) struct Task {
    pub(crate) id: Uuid,
    pub(crate) scheduled_on: Option<Epoch>,
    pub(crate) complete_date: Option<NaiveDate>,
    pub(crate) title: String,
    pub(crate) cost: Option<i32>,
}

impl Task {
    pub(crate) fn is_completed(&self) -> bool {
        self.complete_date.is_some()
    }
}

pub(crate) struct CreateTaskInput {
    pub(crate) scheduled_on: Option<Epoch>,
    pub(crate) title: String,
    pub(crate) cost: Option<i32>,
}

pub(crate) async fn create_task(
    user_id: Uuid,
    input: CreateTaskInput,
    db_conn: &DatabaseConnection,
) -> AppResult<Task> {
    let task_id = Uuid::new_v4();
    let schedule_index_date = input.scheduled_on.map(|e| e.index_date());
    let task = entities::task::ActiveModel {
        id: Set(task_id),
        user_id: Set(user_id),
        scheduled_on: Set(input
            .scheduled_on
            .map(serde_json::to_value)
            .transpose()
            .context("Failed to convert scheduled_on to JSON")?),
        schedule_index_date: Set(schedule_index_date),
        title: Set(input.title),
        cost: Set(input.cost),
        ..Default::default()
    };
    let task = task.insert(db_conn).await?;

    Ok(task
        .try_into()
        .context("Bug: the task just inserted in `create_task` cannot be convert to a Task")?)
}

#[derive(Default)]
pub(crate) struct TaskFilter {
    pub(crate) view_filter: Option<ViewFilter>,
}

pub(crate) struct ViewFilter {
    pub(crate) view_type: ViewType,
    pub(crate) epoch: Option<Epoch>,
}

#[derive(Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
pub(crate) enum ViewType {
    Planned,
    Scheduled,
}

pub(crate) async fn list_tasks(
    user_id: Uuid,
    filter: TaskFilter,
    db_conn: &DatabaseConnection,
) -> AppResult<Vec<Task>> {
    let query = entities::task::Entity::find().filter(entities::task::Column::UserId.eq(user_id));

    let tasks = query
        .all(db_conn)
        .await?
        .into_iter()
        .map(Task::try_from)
        .collect::<Result<_, _>>()?;

    let Some(view_filter) = filter.view_filter else {
        return Ok(tasks);
    };

    fn generalized_contains(e1: Option<Epoch>, e2: Option<Epoch>) -> bool {
        match (e1, e2) {
            (Some(e1), Some(e2)) => e1.contains(e2),
            (None, _) => true,
            (_, None) => false,
        }
    }

    match view_filter.view_type {
        ViewType::Scheduled => {
            let filter = |task: &Task| generalized_contains(view_filter.epoch, task.scheduled_on);
            Ok(tasks.into_iter().filter(filter).collect())
        }
        ViewType::Planned => {
            let filter = |task: &Task| {
                let scheduled_epoch = dbg!(task.scheduled_on);
                let query_epoch = dbg!(view_filter.epoch);

                generalized_contains(task.scheduled_on, view_filter.epoch)
                    && scheduled_epoch.map(|e| e.date_range())
                        != query_epoch.map(|e| e.date_range())
            };
            Ok(tasks.into_iter().filter(filter).collect())
        }
    }
}

#[derive(Default)]
pub(crate) struct UpdateTaskInput {
    pub(crate) id: Uuid,
    pub(crate) scheduled_on: Maybe<Option<Epoch>>,
    pub(crate) complete_date: Maybe<Option<NaiveDate>>,
    pub(crate) title: Maybe<String>,
    pub(crate) cost: Maybe<Option<i32>>,
}

pub(crate) async fn update_task(
    user_id: Uuid,
    input: UpdateTaskInput,
    db_conn: &DatabaseConnection,
) -> AppResult<Task> {
    let id = input.id;
    let tx = db_conn.begin().await?;

    let mut task = entities::task::Entity::find_by_id(id)
        .filter(entities::task::Column::UserId.eq(user_id))
        .one(&tx)
        .await?
        .ok_or_else(|| AppError::task_not_found(id))?
        .into_active_model();
    if let Maybe::Some(scheduled_on) = input.scheduled_on {
        task.schedule_index_date = Set(scheduled_on.map(|e| e.index_date()));
        task.scheduled_on = Set(scheduled_on
            .map(serde_json::to_value)
            .transpose()
            .context("Failed to convert scheduled_on to JSON")?);
    }
    if let Maybe::Some(complete_date) = input.complete_date {
        task.complete_date = Set(complete_date);
    }
    if let Maybe::Some(title) = input.title {
        task.title = Set(title);
    }
    if let Maybe::Some(cost) = input.cost {
        task.cost = Set(cost);
    }
    let task = task.update(db_conn).await?;

    Ok(task.try_into()?)
}

pub(crate) async fn delete_task(
    user_id: Uuid,
    task_id: Uuid,
    db_conn: &DatabaseConnection,
) -> AppResult<()> {
    let entities = entities::task::Entity::delete_by_id(task_id)
        .filter(entities::task::Column::UserId.eq(user_id))
        .exec(db_conn)
        .await?;

    if entities.rows_affected != 1 {
        return Err(AppError::task_not_found(task_id));
    }

    Ok(())
}

impl TryFrom<TaskModel> for Task {
    type Error = anyhow::Error;

    fn try_from(value: TaskModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            scheduled_on: value.scheduled_on.map(serde_json::from_value).transpose()?,
            complete_date: value.complete_date,
            title: value.title,
            cost: value.cost,
        })
    }
}

#[extend::ext]
impl<T> MaybeUndefined<T> {
    fn into_option(self) -> Option<Option<T>> {
        match self {
            MaybeUndefined::Value(x) => Some(Some(x)),
            MaybeUndefined::Null => Some(None),
            MaybeUndefined::Undefined => None,
        }
    }

    fn into_option_nonnull(self) -> Result<Option<T>, ()> {
        match self {
            MaybeUndefined::Value(x) => Ok(Some(x)),
            MaybeUndefined::Null => Err(()),
            MaybeUndefined::Undefined => Ok(None),
        }
    }
}

#[extend::ext]
impl<T> Option<T> {
    fn into_active_value(self) -> ActiveValue<T>
    where
        T: Into<sea_orm::Value>,
    {
        match self {
            Some(x) => Set(x),
            None => NotSet,
        }
    }
}
