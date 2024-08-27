use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entities::{self, task::Model as TaskModel};

use super::{plan::Plan, task::Task, PgLoader};

#[repr(transparent)]
#[derive(
    Copy, Clone, Eq, PartialEq, Hash, Debug, async_graphql::NewType, sea_orm::DeriveValueType,
)]
pub(crate) struct TaskItemId(pub Uuid);

#[derive(Clone, Debug)]
pub(super) enum TaskItem {
    Task(Task),
    Plan(Plan),
}

impl TaskItem {
    fn id(&self) -> Uuid {
        match self {
            TaskItem::Task(task) => task.id,
            TaskItem::Plan(plan) => plan.id,
        }
    }
}

impl TryFrom<TaskModel> for TaskItem {
    type Error = anyhow::Error;

    fn try_from(value: TaskModel) -> Result<Self, Self::Error> {
        if value.schedule_date.is_some() {
            Ok(TaskItem::Task(value.try_into()?))
        } else {
            Ok(TaskItem::Plan(value.try_into()?))
        }
    }
}

impl Loader<TaskItemId> for PgLoader {
    type Value = TaskItem;
    type Error = Arc<anyhow::Error>;

    async fn load(
        &self,
        keys: &[TaskItemId],
    ) -> std::result::Result<HashMap<TaskItemId, TaskItem>, Self::Error> {
        self.load_task_items(keys).await.map_err(Arc::new)
    }
}

impl PgLoader {
    pub(crate) async fn load_task_items(
        &self,
        keys: &[TaskItemId],
    ) -> std::result::Result<HashMap<TaskItemId, TaskItem>, anyhow::Error> {
        // TODO: check user
        Ok(entities::task::Entity::find()
            .filter(entities::task::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db_conn)
            .await?
            .into_iter()
            .map(|model| {
                model
                    .try_into()
                    .map(|task: TaskItem| (TaskItemId(task.id()), task))
            })
            .collect::<Result<_, _>>()?)
    }

    // pub(crate) async fn load_tasks(
    //     &self,
    //     keys: &[TaskItemId],
    // ) -> std::result::Result<HashMap<TaskItemId, Task>, anyhow::Error> {
    // TODO: check user
    // Ok(entities::tasks::Entity::find()
    //     .filter(entities::tasks::Column::Id.is_in(keys.iter().copied()))
    //     .find_with_related(entities::iterations_tasks::Entity)
    //     .all(&self.db_conn)
    //     .await?
    //     .into_iter()
    //     .map(|(model, rel)| {
    //         let task = Task::from_task_and_relation_models(model, rel);
    //
    //         (TaskId(task.id), task)
    //     })
    //     .collect())

    // Ok(entities::task::Entity::find()
    //     .filter(entities::task::Column::Id.is_in(keys.iter().copied()))
    //     .all(&self.db_conn)
    //     .await?
    //     .into_iter()
    //     .map(|model| Task::try_from(model).map(|task| (TaskItemId(task.id), task)))
    //     .collect::<Result<_, _>>()?)
    // }
}
