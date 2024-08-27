use anyhow::{bail, Context as _};
use async_graphql::{Json, MaybeUndefined};
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, task::Model as TaskModel};

use super::{task_item::TaskItem, AppError, AppResult};

#[derive(Clone, Debug)]
pub(super) struct Plan {
    pub(super) id: Uuid,
    recurring_spec: Option<RecurringSpec>,
    title: String,
    cost: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum RecurringSpec {
    Weekly,
}

#[derive(async_graphql::InputObject)]
struct CreatePlanInput {
    recurring_spec: Json<RecurringSpec>,
    title: String,
    cost: Option<i32>,
}

pub(super) async fn create_plan(
    user_id: Uuid,
    input: CreatePlanInput,
    db_conn: &DatabaseConnection,
) -> AppResult<Plan> {
    todo!()
}

pub(super) async fn get_plan(
    user_id: Uuid,
    plan_id: Uuid,
    db_conn: &DatabaseConnection,
) -> AppResult<Plan> {
    Ok(entities::task::Entity::find()
        .filter(entities::task::Column::Id.eq(plan_id))
        .filter(entities::task::Column::UserId.eq(user_id))
        .one(db_conn)
        .await?
        .ok_or(AppError::ResourceNotFound(plan_id))?
        .try_into()?)
}

pub(super) async fn list_plans(
    user_id: Uuid,
    db_conn: &DatabaseConnection,
) -> AppResult<Vec<Plan>> {
    let query = entities::task::Entity::find().filter(entities::task::Column::UserId.eq(user_id));

    Ok(query
        .all(db_conn)
        .await?
        .into_iter()
        .map(|model| model.try_into())
        .collect::<Result<_, _>>()?)
}

#[derive(async_graphql::InputObject, Default)]
struct UpdatePlanInput {
    id: Uuid,
    schedule_date: MaybeUndefined<NaiveDate>,
    complete_date: MaybeUndefined<NaiveDate>,
    title: MaybeUndefined<String>,
    cost: MaybeUndefined<i32>,
}

pub(super) async fn update_plan(
    user_id: Uuid,
    plan_id: Uuid,
    input: UpdatePlanInput,
    db_conn: &DatabaseConnection,
) -> AppResult<Plan> {
    todo!()
}

pub(super) async fn delete_plan(
    user_id: Uuid,
    plan_id: Uuid,
    db_conn: &DatabaseConnection,
) -> AppResult<()> {
    todo!()
}

impl TryFrom<TaskModel> for Plan {
    type Error = anyhow::Error;

    fn try_from(value: TaskModel) -> Result<Self, Self::Error> {
        if value.schedule_date.is_some() {
            bail!("Expect schedule_date to be None for a plan, but it is scheduled")
        }
        Ok(Self {
            id: value.id,
            recurring_spec: value
                .recurring_spec
                .map(|v| serde_json::from_value(v))
                .transpose()
                .context("Failed to parse recurring_spec")?,
            title: value.title,
            cost: value.cost,
        })
    }
}

impl TryFrom<TaskItem> for Plan {
    type Error = anyhow::Error;

    fn try_from(value: TaskItem) -> Result<Self, Self::Error> {
        match value {
            TaskItem::Task(..) => bail!("Expect Plan, but got Task"),
            TaskItem::Plan(v) => Ok(v),
        }
    }
}
