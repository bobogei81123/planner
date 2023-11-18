use std::{
    collections::HashMap,
    ops::{Bound, RangeBounds},
    sync::Arc,
};

use anyhow::anyhow;
use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{postgres::types::PgRange, PgPool};
use uuid::Uuid;

use super::{
    task::{Task, TaskId},
    PgLoader, Result,
};

#[derive(Clone, Debug, sqlx::FromRow, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Iteration {
    id: Uuid,
    name: String,
    #[graphql(skip)]
    date_range: Option<PgRange<NaiveDate>>,
    #[graphql(skip)]
    tasks: Vec<Uuid>,
}

#[async_graphql::ComplexObject]
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

    async fn tasks(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Task>> {
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
pub(crate) struct IterationId(pub Uuid);

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

pub(crate) async fn get_all_iterations(
    user_id: Uuid,
    ctx: &async_graphql::Context<'_>,
) -> Result<Vec<Iteration>> {
    let iters = sqlx::query_as!(
        Iteration,
        r#"SELECT iterations.id, iterations.name, iterations.date_range,
                   array_remove(array_agg(iterations_tasks.task_id), NULL) AS "tasks!"
               FROM iterations
               LEFT JOIN iterations_tasks ON iterations.id = iterations_tasks.iteration_id
               WHERE iterations.user_id = $1
               GROUP BY iterations.id; "#,
        user_id,
    )
    .fetch_all(ctx.data_unchecked::<PgPool>())
    .await?
    .into_iter()
    .collect();

    Ok(iters)
}
