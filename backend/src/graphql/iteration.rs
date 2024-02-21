use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entities;

use super::{
    task::{Task, TaskId},
    AppResult, DbResult, PgLoader,
};

#[repr(transparent)]
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    sqlx::Type,
    async_graphql::NewType,
    sea_orm::DeriveValueType,
)]
pub(crate) struct IterationId(pub Uuid);

#[derive(Clone, Debug, sqlx::FromRow, async_graphql::SimpleObject)]
#[graphql(complex)]
pub(super) struct Iteration {
    id: Uuid,
    name: String,
    #[graphql(skip)]
    tasks: Vec<TaskId>,
}

#[async_graphql::ComplexObject]
impl Iteration {
    async fn tasks(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Task>> {
        let loader = ctx.data_unchecked::<DataLoader<PgLoader>>();
        let tasks = loader.load_many(self.tasks.iter().copied()).await?;

        Ok(tasks.into_values().collect())
    }
}

impl Iteration {
    fn from_model_and_task_ids(model: entities::iterations::Model, task_ids: Vec<TaskId>) -> Self {
        let mut iteration: Iteration = model.into();
        iteration.tasks = task_ids;

        iteration
    }
}

impl From<entities::iterations::Model> for Iteration {
    fn from(entities::iterations::Model { id, name, .. }: entities::iterations::Model) -> Self {
        Self {
            id,
            name,
            tasks: vec![],
        }
    }
}

#[async_trait]
impl Loader<IterationId> for PgLoader {
    type Value = Iteration;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[IterationId],
    ) -> std::result::Result<HashMap<IterationId, Iteration>, Self::Error> {
        self.load_iterations(keys).await.map_err(Arc::new)
    }
}

impl PgLoader {
    async fn load_iterations(
        &self,
        keys: &[IterationId],
    ) -> DbResult<HashMap<IterationId, Iteration>> {
        // TODO: check user
        Ok(entities::iterations::Entity::find()
            .filter(entities::iterations::Column::Id.is_in(keys.iter().map(|id| id.0)))
            .find_with_related(entities::iterations_tasks::Entity)
            .all(&self.db_conn)
            .await
            .unwrap()
            .into_iter()
            .map(|(model, rel)| {
                let iteration = Iteration::from_model_and_task_ids(
                    model,
                    rel.into_iter().map(|rel| TaskId(rel.task_id)).collect(),
                );

                (IterationId(iteration.id), iteration)
            })
            .collect())
    }
}

pub(super) async fn list_iterations(
    user_id: Uuid,
    db_conn: &DatabaseConnection,
) -> AppResult<Vec<Iteration>> {
    Ok(entities::iterations::Entity::find()
        .filter(entities::iterations::Column::UserId.eq(user_id))
        .find_with_related(entities::iterations_tasks::Entity)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|(model, rel)| {
            Iteration::from_model_and_task_ids(
                model,
                rel.into_iter().map(|rel| TaskId(rel.task_id)).collect(),
            )
        })
        .collect())
}
