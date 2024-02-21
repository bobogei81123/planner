use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    MaybeUndefined,
};
use async_trait::async_trait;
use chrono::Duration;
use extend::ext;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, RuntimeErr, Set, TransactionTrait,
    Unset,
};
use uuid::Uuid;

use crate::{db::TransactionExt, entities};

use super::{
    iteration::{Iteration, IterationId},
    AppError, AppResult, BadRequestReason, CreateTaskInput, DateRange, PgLoader, TaskFilter,
    UpdateTaskInput,
};

#[repr(transparent)]
#[derive(
    Copy, Clone, Eq, PartialEq, Hash, Debug, async_graphql::NewType, sea_orm::DeriveValueType,
)]
pub(crate) struct TaskId(pub Uuid);

#[derive(Clone, Debug, async_graphql::SimpleObject)]
#[graphql(complex)]
pub(super) struct Task {
    id: Uuid,
    pub(super) title: String,
    pub(super) status: TaskStatus,
    pub(super) point: Option<i32>,
    #[graphql(skip)]
    iterations: Vec<IterationId>,
    pub(super) planned_on: Option<chrono::NaiveDate>,
}

#[async_graphql::ComplexObject]
impl Task {
    async fn iterations(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<Iteration>> {
        let loader = ctx.data_unchecked::<DataLoader<PgLoader>>();
        let iterations = loader.load_many(self.iterations.iter().copied()).await?;

        Ok(iterations.into_values().collect())
    }
}

impl From<entities::tasks::Model> for Task {
    fn from(
        entities::tasks::Model {
            id,
            title,
            status,
            point,
            planned_on,
            ..
        }: entities::tasks::Model,
    ) -> Self {
        Self {
            id,
            title,
            status: status.into(),
            point,
            iterations: Vec::new(),
            planned_on,
        }
    }
}

impl Task {
    fn from_model_and_iteration_ids(
        model: entities::tasks::Model,
        iter_ids: Vec<IterationId>,
    ) -> Self {
        let mut task: Task = model.into();
        task.iterations = iter_ids;

        task
    }

    fn from_task_and_relation_models(
        model: entities::tasks::Model,
        rel_models: Vec<entities::iterations_tasks::Model>,
    ) -> Self {
        Task::from_model_and_iteration_ids(
            model,
            rel_models
                .into_iter()
                .map(|rel| IterationId(rel.iteration_id))
                .collect(),
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, sqlx::Type, async_graphql::Enum)]
pub(crate) enum TaskStatus {
    Active,
    Completed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl From<entities::sea_orm_active_enums::TaskStatus> for TaskStatus {
    fn from(value: entities::sea_orm_active_enums::TaskStatus) -> Self {
        match value {
            entities::sea_orm_active_enums::TaskStatus::Active => TaskStatus::Active,
            entities::sea_orm_active_enums::TaskStatus::Completed => TaskStatus::Completed,
        }
    }
}

impl From<TaskStatus> for entities::sea_orm_active_enums::TaskStatus {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Active => entities::sea_orm_active_enums::TaskStatus::Active,
            TaskStatus::Completed => entities::sea_orm_active_enums::TaskStatus::Completed,
        }
    }
}

#[async_trait]
impl Loader<TaskId> for PgLoader {
    type Value = Task;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[TaskId],
    ) -> std::result::Result<HashMap<TaskId, Task>, Self::Error> {
        self.load_tasks(keys).await.map_err(Arc::new)
    }
}

impl PgLoader {
    pub(crate) async fn load_tasks(
        &self,
        keys: &[TaskId],
    ) -> std::result::Result<HashMap<TaskId, Task>, DbErr> {
        // TODO: check user
        Ok(entities::tasks::Entity::find()
            .filter(entities::tasks::Column::Id.is_in(keys.iter().copied()))
            .find_with_related(entities::iterations_tasks::Entity)
            .all(&self.db_conn)
            .await?
            .into_iter()
            .map(|(model, rel)| {
                let task = Task::from_task_and_relation_models(model, rel);

                (TaskId(task.id), task)
            })
            .collect())
    }
}

pub(super) async fn list_tasks(
    user_id: Uuid,
    filter: TaskFilter,
    db_conn: &DatabaseConnection,
) -> AppResult<Vec<Task>> {
    let mut query =
        entities::tasks::Entity::find().filter(entities::tasks::Column::UserId.eq(user_id));
    if let Some(date_range) = filter.planned_date_range {
        let DateRange { start, end } = date_range;
        if start > end {
            return Err(AppError::BadRequest(BadRequestReason::InvalidDateRange))?;
        }
        query = query
            .filter(entities::tasks::Column::PlannedOn.between(start, end - Duration::days(1)));
    }

    Ok(query
        .find_with_related(entities::iterations_tasks::Entity)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|(model, rel)| Task::from_task_and_relation_models(model, rel))
        .collect())
}

pub(super) async fn create_task(
    user_id: Uuid,
    input: CreateTaskInput,
    db_conn: &DatabaseConnection,
) -> AppResult<Task> {
    let task_id = Uuid::new_v4();
    let tx = db_conn.begin().await?;

    let task = tx
        .with(move |tx| async move {
            let tx = tx.as_ref();
            create_task_in_transaction(user_id, input, tx).await
        })
        .await?;

    Ok(task)
}

pub(super) async fn create_task_in_transaction(
    user_id: Uuid,
    input: CreateTaskInput,
    tx: &DatabaseTransaction,
) -> AppResult<Task> {
    let task_id = Uuid::new_v4();
    let task = entities::tasks::ActiveModel {
        id: Set(task_id),
        user_id: Set(user_id),
        title: Set(input.title),
        status: Set(TaskStatus::Active.into()),
        point: Set(input.point),
        planned_on: Set(input.planned_on),
    };
    let task = task.insert(tx).await?;

    if let Some(iteration) = input.iteration {
        let result = entities::iterations_tasks::ActiveModel {
            iteration_id: Set(iteration),
            task_id: Set(task_id),
        }
        .insert(tx)
        .await;

        match result {
            Err(DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))))
                if db_err
                    .constraint()
                    .is_some_and(|x| x == "iterations_tasks_iteration_id_fkey") =>
            {
                return Err(AppError::ResourceNotFound(iteration))
            }
            Err(err) => return Err(err)?,
            Ok(_) => (),
        }
    }

    let iterations = task
        .find_related(entities::iterations_tasks::Entity)
        .all(tx)
        .await?;
    Ok(Task::from_task_and_relation_models(task, iterations))
}

// fn set_by_may_be_undefined<T, U>(target: &mut ActiveValue<Option<T>>, maybe: MaybeUndefined<U>)
// where
//     T: From<U>,
// {
//     match maybe {
//         MaybeUndefined::Value(x) => *target = Set(Some(x.into())),
//         MaybeUndefined::Null => *target = Set(Some(option))
//         MaybeUndefined::Undefined => (),
//     }
// }

#[ext]
impl<T> ActiveValue<Option<T>>
where
    Option<T>: Into<sea_orm::Value>,
{
    fn set_with_maybe_undefined<U>(&mut self, maybe: MaybeUndefined<U>)
    where
        T: From<U>,
    {
        match maybe {
            MaybeUndefined::Value(x) => *self = Set(Some(x.into())),
            MaybeUndefined::Null => *self = Set(None),
            MaybeUndefined::Undefined => (),
        }
    }
}

#[ext]
impl<T> ActiveValue<T>
where
    T: Into<sea_orm::Value>,
{
    fn set_with_maybe_undefined_non_null<U>(&mut self, maybe: MaybeUndefined<U>)
    where
        T: From<U>,
    {
        match maybe {
            MaybeUndefined::Value(x) => *self = Set(x.into()),
            MaybeUndefined::Null | MaybeUndefined::Undefined => (),
        }
    }
}

pub(super) async fn update_task(
    user_id: Uuid,
    input: UpdateTaskInput,
    db_conn: &DatabaseConnection,
) -> AppResult<Task> {
    let id = input.id;
    let tx = db_conn.begin().await?;

    let task = tx
        .with(|tx| async move {
            let tx = tx.as_ref();

            let mut task = entities::tasks::Entity::find_by_id(id)
                .filter(entities::tasks::Column::UserId.eq(user_id))
                .one(tx)
                .await?
                .ok_or_else(|| AppError::ResourceNotFound(id))?
                .into_active_model();
            task.title.set_with_maybe_undefined_non_null(input.title);
            task.status.set_with_maybe_undefined_non_null(input.status);
            task.point.set_with_maybe_undefined(input.point);
            task.planned_on.set_with_maybe_undefined(input.planned_on);
            let task = task.update(db_conn).await?;

            if let MaybeUndefined::Value(_) | MaybeUndefined::Null = &input.iterations {
                entities::iterations_tasks::Entity::delete_many()
                    .filter(entities::iterations_tasks::Column::TaskId.eq(id))
                    .exec(tx)
                    .await?;

                if let MaybeUndefined::Value(iteration) = input.iterations {
                    entities::iterations_tasks::Entity::insert_many(
                        iteration
                            .into_iter()
                            .map(|iter_id| entities::iterations_tasks::ActiveModel {
                                iteration_id: Set(iter_id),
                                task_id: Set(id),
                            })
                            .collect::<Vec<_>>(),
                    )
                    .exec(tx)
                    .await?;
                }
            }

            let iterations = task
                .find_related(entities::iterations_tasks::Entity)
                .all(tx)
                .await?;
            Ok::<_, AppError>(Task::from_task_and_relation_models(task, iterations))
        })
        .await?;

    Ok(task)
}

pub(super) async fn delete_task(
    user_id: Uuid,
    task_id: TaskId,
    db_conn: &DatabaseConnection,
) -> AppResult<()> {
    let entities = entities::tasks::Entity::delete_by_id(task_id)
        .filter(entities::tasks::Column::UserId.eq(user_id))
        .exec(db_conn)
        .await?;

    if entities.rows_affected != 1 {
        return Err(AppError::ResourceNotFound(task_id.0));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use googletest::prelude::*;

    use super::*;
    use testlib::{test_uuid, PgDocker, Result};

    const DEFAULT_USER_UUID: Uuid = test_uuid(314159);
    const NOT_DEFAULT_USER_UUID: Uuid = test_uuid(141421);

    async fn insert_default_user(db_conn: &DatabaseConnection) {
        entities::users::ActiveModel {
            id: Set(DEFAULT_USER_UUID),
            username: Set("meteor".to_owned()),
        }
        .insert(db_conn)
        .await
        .expect("cannot insert default user");
    }

    fn default_task() -> entities::tasks::ActiveModel {
        entities::tasks::ActiveModel {
            user_id: Set(DEFAULT_USER_UUID),
            status: Set(entities::sea_orm_active_enums::TaskStatus::Active),
            ..Default::default()
        }
    }

    fn create_loader(db_conn: &DatabaseConnection) -> DataLoader<PgLoader> {
        DataLoader::new(
            PgLoader {
                db_conn: db_conn.clone(),
            },
            tokio::spawn,
        )
    }

    #[googletest::test]
    #[tokio::test]
    async fn loader_can_load_tasks() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        entities::tasks::ActiveModel {
            id: Set(test_uuid(1)),
            title: Set("test #1".to_owned()),
            status: Set(entities::sea_orm_active_enums::TaskStatus::Active),
            point: Set(Some(1)),
            ..default_task()
        }
        .insert(db_conn)
        .await
        .unwrap();
        entities::tasks::ActiveModel {
            id: Set(test_uuid(2)),
            title: Set("test #2".to_owned()),
            status: Set(entities::sea_orm_active_enums::TaskStatus::Active),
            point: Set(Some(2)),
            ..default_task()
        }
        .insert(db_conn)
        .await
        .unwrap();
        entities::tasks::ActiveModel {
            id: Set(test_uuid(3)),
            title: Set("test #3".to_owned()),
            status: Set(entities::sea_orm_active_enums::TaskStatus::Completed),
            point: Set(None),
            ..default_task()
        }
        .insert(db_conn)
        .await
        .unwrap();
        let loader = create_loader(db_conn);

        let result = loader
            .load_many([TaskId(test_uuid(1)), TaskId(test_uuid(3))])
            .await?;

        expect_that!(
            result,
            unordered_elements_are![
                (
                    eq(TaskId(test_uuid(1))),
                    pat!(Task {
                        id: eq(test_uuid(1)),
                        title: eq("test #1"),
                        status: eq(TaskStatus::Active),
                        point: some(eq(1)),
                        iterations: empty(),
                    })
                ),
                (
                    eq(TaskId(test_uuid(3))),
                    pat!(Task {
                        id: eq(test_uuid(3)),
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
    async fn get_tasks_with_planned_date_in_range() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        entities::tasks::ActiveModel {
            id: Set(test_uuid(1)),
            title: Set("test #1".to_owned()),
            planned_on: Set(Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())),
            ..default_task()
        }
        .insert(db_conn)
        .await?;
        entities::tasks::ActiveModel {
            id: Set(test_uuid(2)),
            title: Set("test #2".to_owned()),
            planned_on: Set(Some(NaiveDate::from_ymd_opt(2024, 1, 7).unwrap())),
            ..default_task()
        }
        .insert(db_conn)
        .await?;
        entities::tasks::ActiveModel {
            id: Set(test_uuid(3)),
            title: Set("test #3".to_owned()),
            planned_on: Set(Some(NaiveDate::from_ymd_opt(2024, 1, 5).unwrap())),
            ..default_task()
        }
        .insert(db_conn)
        .await?;

        let result = list_tasks(
            DEFAULT_USER_UUID,
            TaskFilter {
                planned_date_range: Some(DateRange {
                    start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    end: NaiveDate::from_ymd_opt(2024, 1, 6).unwrap(),
                }),
            },
            db_conn,
        )
        .await?;

        expect_that!(
            result,
            unordered_elements_are![
                pat!(Task {
                    id: eq(test_uuid(1)),
                    title: eq("test #1"),
                }),
                pat!(Task {
                    id: eq(test_uuid(3)),
                    title: eq("test #3"),
                }),
            ]
        );
        Ok(())
    }

    #[googletest::test]
    #[tokio::test]
    async fn update_task_can_update_title() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        let loader = create_loader(db_conn);
        entities::tasks::ActiveModel {
            id: Set(test_uuid(1)),
            title: Set("title".to_owned()),
            ..default_task()
        }
        .insert(db_conn)
        .await?;

        let update_result = update_task(
            DEFAULT_USER_UUID,
            UpdateTaskInput {
                id: test_uuid(1),
                title: MaybeUndefined::Value("updated title".to_owned()),
                ..Default::default()
            },
            db_conn,
        )
        .await?;
        let confirm_result = loader.load_one(TaskId(test_uuid(1))).await?;

        expect_that!(
            update_result,
            pat!(Task {
                id: eq(test_uuid(1)),
                title: eq("updated title"),
            })
        );
        expect_that!(
            confirm_result,
            some(pat!(Task {
                id: eq(test_uuid(1)),
                title: eq("updated title"),
            }))
        );
        Ok(())
    }

    #[googletest::test]
    #[tokio::test]
    async fn when_user_id_not_match_in_update_task_returns_not_found() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        let loader = create_loader(db_conn);
        entities::tasks::ActiveModel {
            id: Set(test_uuid(1)),
            title: Set("old title".to_owned()),
            ..default_task()
        }
        .insert(db_conn)
        .await?;

        let result = update_task(
            NOT_DEFAULT_USER_UUID,
            UpdateTaskInput {
                id: test_uuid(1),
                title: MaybeUndefined::Value("updated title".to_owned()),
                ..Default::default()
            },
            db_conn,
        )
        .await;
        let confirm_result = loader.load_one(TaskId(test_uuid(1))).await?;

        expect_that!(
            result,
            err(pat!(AppError::ResourceNotFound(eq(test_uuid(1)))))
        );
        expect_that!(
            confirm_result,
            some(pat!(Task {
                title: eq("old title")
            }))
        );
        Ok(())
    }

    #[googletest::test]
    #[tokio::test]
    async fn delete_task_can_delete_task() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        let loader = create_loader(db_conn);
        entities::tasks::ActiveModel {
            id: Set(test_uuid(1)),
            title: Set("title".to_owned()),
            ..default_task()
        }
        .insert(db_conn)
        .await?;
        loader
            .load_one(TaskId(test_uuid(1)))
            .await
            .unwrap()
            .expect("there should be one task after we inserted one");

        let result = delete_task(DEFAULT_USER_UUID, TaskId(test_uuid(1)), db_conn).await;
        let confirm_result = loader.load_one(TaskId(test_uuid(1))).await;

        expect_that!(result, ok(()));
        expect_that!(confirm_result, ok(none()));

        Ok(())
    }

    #[googletest::test]
    #[tokio::test]
    async fn when_user_id_not_match_in_delete_task_returns_not_found() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        let loader = create_loader(db_conn);
        entities::tasks::ActiveModel {
            id: Set(test_uuid(1)),
            title: Set("title".to_owned()),
            ..default_task()
        }
        .insert(db_conn)
        .await?;
        loader
            .load_one(TaskId(test_uuid(1)))
            .await
            .unwrap()
            .expect("there should be one task after we inserted one");

        let result = delete_task(NOT_DEFAULT_USER_UUID, TaskId(test_uuid(1)), db_conn).await;
        let confirm_result = loader.load_one(TaskId(test_uuid(1))).await?;

        expect_that!(
            result,
            err(pat!(AppError::ResourceNotFound(eq(test_uuid(1)))))
        );
        expect_that!(
            confirm_result,
            some(pat!(Task {
                id: eq(test_uuid(1)),
                title: eq("title")
            }))
        );

        Ok(())
    }
}
