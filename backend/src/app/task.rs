use anyhow::Context;
use async_graphql::MaybeUndefined;
use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};
use futures::FutureExt;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, NotSet},
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set, TransactionTrait,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    app::time::EpochLike,
    db::DatabaseTransactionExt,
    entities::{self, task::Model as TaskModel},
};

use super::{
    maybe::Maybe,
    time::{today, Epoch, EpochKind, RecurringPattern, RecurringSpec},
    AppError, AppResult,
};

#[derive(Clone, Debug)]
pub(crate) struct Task {
    pub(crate) id: Uuid,
    pub(crate) scheduled_on: Option<Epoch>,
    pub(crate) complete_date: Option<NaiveDate>,
    pub(crate) recurring_data: Option<RecurringData>,
    pub(crate) title: String,
    pub(crate) cost: Option<i32>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct RecurringData {
    next_check_date: NaiveDate,
    pub(crate) spec: RecurringSpec,
}

impl Task {
    pub(crate) fn is_completed(&self) -> bool {
        self.complete_date.is_some()
    }

    fn into_active_model(self) -> AppResult<entities::task::ActiveModel> {
        let scheduled_on = self
            .scheduled_on
            .map(serde_json::to_value)
            .transpose()
            .map_err(|err| {
                AppError::invalid_input(format!("failed to convert `scheduled_on` to JSON: {err}"))
            })?;
        let (next_recurring_check_date, recurring_spec) = match &self.recurring_data {
            Some(RecurringData {
                next_check_date,
                spec,
            }) => {
                let spec_json = serde_json::to_value(spec).map_err(|err| {
                    AppError::invalid_input(format!(
                        "failed to convert `recurring_data.spec` to JSON: {err}"
                    ))
                })?;

                (Some(*next_check_date), Some(spec_json))
            }
            None => (None, None),
        };

        Ok(entities::task::ActiveModel {
            id: Set(self.id),
            scheduled_on: Set(scheduled_on),
            next_recurring_check_date: Set(next_recurring_check_date),
            recurring_spec: Set(recurring_spec),
            title: Set(self.title),
            cost: Set(self.cost),
            complete_date: Set(self.complete_date),
            ..Default::default()
        })
    }

    async fn schedule_recurring_until(
        &mut self,
        user_id: Uuid,
        until: NaiveDate,
        db_conn: &impl ConnectionTrait,
    ) -> AppResult<()> {
        let Some(recurring_data) = &mut self.recurring_data else {
            warn!("[BUG] Schedule_next_recurring called on a non-recurring task.");
            return Ok(());
        };

        let mut next_schedule_epoch = recurring_data
            .spec
            .next_starting_from(recurring_data.next_check_date);
        while next_schedule_epoch.start_date() < until {
            let mut child_task_model = Task {
                id: Uuid::new_v4(),
                scheduled_on: Some(next_schedule_epoch),
                complete_date: None,
                recurring_data: None,
                title: self.title.clone(),
                cost: self.cost,
            }
            .into_active_model()?;
            child_task_model.user_id = Set(user_id);

            let child_task = child_task_model.insert(db_conn).await?;
            info!(epoch = ?child_task.scheduled_on, id = ?child_task.id, "Scheduled recurring task");

            next_schedule_epoch = recurring_data
                .spec
                .next_starting_from(next_schedule_epoch.end_date());
        }

        recurring_data.next_check_date = next_schedule_epoch.start_date();
        Ok(())
    }

    async fn save_next_check_date(&mut self, db: &impl ConnectionTrait) -> AppResult<()> {
        let Some(recurring_data) = &self.recurring_data else {
            warn!("[BUG] save_next_check_date called on a non-recurring task.");
            return Ok(());
        };
        let model = entities::task::ActiveModel {
            id: Set(self.id),
            next_recurring_check_date: Set(Some(recurring_data.next_check_date)),
            ..Default::default()
        }
        .into_active_model();
        model.update(db).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct CreateTaskInput {
    pub(crate) scheduled_on: Option<Epoch>,
    pub(crate) recurring_spec: Option<RecurringSpec>,
    pub(crate) title: String,
    pub(crate) cost: Option<i32>,
}

pub(crate) async fn create_task(
    user_id: Uuid,
    input: CreateTaskInput,
    db_conn: &DatabaseConnection,
) -> AppResult<Task> {
    info!(?input, "Create new task");
    let task_id = Uuid::new_v4();
    if let Some(recurring_spec) = &input.recurring_spec {
        validate_recurring_spec(recurring_spec)?;
    }
    let mut task = Task {
        id: task_id,
        scheduled_on: input.scheduled_on,
        complete_date: None,
        recurring_data: input.recurring_spec.map(|spec| RecurringData {
            next_check_date: today(),
            spec,
        }),
        title: input.title,
        cost: input.cost,
    };

    let tx = db_conn.begin().await?;

    let task = tx
        .with(|tx| async move {
            if task.recurring_data.is_some() {
                task.schedule_recurring_until(user_id, today() + TimeDelta::days(14), &*tx)
                    .await?;
            }
            let mut task = task.into_active_model()?;
            task.user_id = Set(user_id);
            let task = task.insert(&*tx).await?;

            Ok::<_, AppError>(task)
        })
        .await?;

    Ok(task
        .try_into()
        .context("Bug: the task just inserted in `create_task` cannot be convert to a Task")?)
}

fn validate_recurring_spec(spec: &RecurringSpec) -> AppResult<()> {
    if let RecurringPattern::EveryEpoch {
        kind: EpochKind::Week,
        ..
    } = spec.pattern
    {
        if spec.start_date.weekday() != Weekday::Mon {
            return Err(AppError::invalid_input(
                "for recurring spec that repeats every week, the start date must be on Monday",
            ));
        }
    }

    Ok(())
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
    Scheduled,
    Planned,
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
            let filter = |task: &Task| generalized_contains(task.scheduled_on, view_filter.epoch);
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

pub async fn schedule_all_recurring_tasks_until(
    db_conn: &DatabaseConnection,
    until: Option<NaiveDate>,
) -> AppResult<()> {
    let until = until.unwrap_or_else(|| today() + TimeDelta::days(14));
    info!(?until, "Schedule all recurring tasks until {until:?}");

    db_conn
        .transaction(|tx| {
            async move {
                let recurring_tasks = entities::task::Entity::find()
                    .filter(
                        entities::task::Column::RecurringSpec
                            .is_not_null()
                            .and(entities::task::Column::CompleteDate.is_null()),
                    )
                    .all(tx)
                    .await?;

                for task in recurring_tasks {
                    let user_id = task.user_id;
                    let mut task: Task = task.try_into()?;
                    task.schedule_recurring_until(user_id, until, tx).await?;
                    task.save_next_check_date(tx).await?;
                }

                Ok::<_, AppError>(())
            }
            .boxed()
        })
        .await?;

    Ok(())
}

impl TryFrom<TaskModel> for Task {
    type Error = anyhow::Error;

    fn try_from(value: TaskModel) -> Result<Self, Self::Error> {
        let recurring_data = match (value.next_recurring_check_date, value.recurring_spec) {
            (None, Some(_)) | (Some(_), None) => {
                anyhow::bail!(
                    "`next_recurring_check_date` and `recurring_spec` must both be set or unset"
                );
            }
            (None, None) => None,
            (Some(next_recurring_check_date), Some(recurring_spec)) => Some(RecurringData {
                next_check_date: next_recurring_check_date,
                spec: serde_json::from_value(recurring_spec)?,
            }),
        };

        Ok(Self {
            id: value.id,
            scheduled_on: value.scheduled_on.map(serde_json::from_value).transpose()?,
            recurring_data,
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
