use anyhow::{anyhow, Context};
use async_graphql::ComplexObject;
use chrono::{Datelike, Duration, NaiveDate, Weekday};

#[cfg(not(test))]
use chrono::Local;
#[cfg(test)]
use testlib::time::Local;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction,
    EntityTrait, IntoActiveModel, QueryFilter, TransactionTrait,
};
use uuid::Uuid;

use crate::{db::TransactionExt, entities};

use super::{
    task::create_task_in_transaction, AppError, AppResult, BadRequestReason, CreateTaskInput,
    CreateTaskScheduleInput,
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
pub(crate) struct TaskScheduleId(pub Uuid);

#[derive(Clone, Debug, async_graphql::SimpleObject, serde::Deserialize)]
#[graphql(complex)]
pub(crate) struct TaskSchedule {
    id: Uuid,
    user_id: Uuid,
    #[graphql(skip)]
    date_spec: DateSpec,
    next_date_to_check: NaiveDate,
    task_title: String,
    task_point: Option<i32>,
}

#[ComplexObject]
impl TaskSchedule {
    async fn date_spec(&self) -> async_graphql::Json<DateSpec> {
        async_graphql::Json(self.date_spec.clone())
    }
}

pub(super) fn random_task_schedule() -> TaskSchedule {
    TaskSchedule {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        date_spec: DateSpec::RepeatsWeekly(WeeklySpec {
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            week_days: vec![Weekday::Mon, Weekday::Tue],
            every_n_week: 1,
        }),
        next_date_to_check: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        task_title: String::new(),
        task_point: None,
    }
}

impl TaskSchedule {
    pub(crate) async fn schedule_until(
        &mut self,
        end_date: NaiveDate,
        tx: &DatabaseTransaction,
    ) -> AppResult<()> {
        for date in self
            .next_date_to_check
            .iter_days()
            .take_while(|d| *d < end_date)
        {
            if self.date_spec.should_schedule(date) {
                create_task_in_transaction(
                    self.user_id,
                    CreateTaskInput {
                        title: self.task_title.clone(),
                        planned_on: Some(date),
                        point: self.task_point,
                        iteration: None,
                    },
                    tx,
                )
                .await?;
            }
        }
        Ok(())
    }
}

impl TryFrom<entities::task_schedule::Model> for TaskSchedule {
    type Error = anyhow::Error;

    fn try_from(value: entities::task_schedule::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            user_id: value.user_id,
            date_spec: serde_json::from_value(value.date_spec)
                .context("failed to parse date spec into internal representation")?,
            next_date_to_check: value.next_date_to_check,
            task_title: value.task_title,
            task_point: value.task_point,
        })
    }
}

impl TryFrom<TaskSchedule> for entities::task_schedule::Model {
    type Error = anyhow::Error;

    fn try_from(value: TaskSchedule) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            user_id: value.user_id,
            date_spec: serde_json::to_value(value.date_spec)
                .context("bug: failed to convert date spec into JSON object.")?,
            next_date_to_check: value.next_date_to_check,
            task_title: value.task_title,
            task_point: value.task_point,
        })
    }

    // fn try_from(value: entities::task_schedule::Model) -> Result<Self, Self::Error> {
    // }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub(super) enum DateSpec {
    // RepeatsDaily(DailySpec),
    RepeatsWeekly(WeeklySpec),
    // RepeatsMonthly(MonthlySpec),
}

impl DateSpec {
    fn should_schedule(&self, date: NaiveDate) -> bool {
        match self {
            DateSpec::RepeatsWeekly(spec) => spec.should_schedule(date),
        }
    }
}

trait ScheduleSpec {
    fn should_schedule(&self, date: NaiveDate) -> bool;
}

// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
// struct DailySpec {
//     start_date: NaiveDate,
// }

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct WeeklySpec {
    week_days: Vec<Weekday>,
    start_date: NaiveDate,
    every_n_week: u32,
}

impl ScheduleSpec for WeeklySpec {
    fn should_schedule(&self, date: NaiveDate) -> bool {
        fn first_date_in_same_week(date: NaiveDate) -> NaiveDate {
            date.week(Weekday::Mon).first_day()
        }
        return self.start_date <= date
            && self.week_days.contains(&date.weekday())
            && (first_date_in_same_week(date) - first_date_in_same_week(self.start_date))
                .num_weeks()
                % self.every_n_week as i64
                == 0;
    }
}

pub(super) async fn list_task_schedules(
    user_id: Uuid,
    db_conn: &DatabaseConnection,
) -> AppResult<Vec<TaskSchedule>> {
    Ok(entities::task_schedule::Entity::find()
        .filter(entities::task_schedule::Column::UserId.eq(user_id))
        .all(db_conn)
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?)
}

pub(super) async fn create_task_schedule(
    user_id: Uuid,
    input: CreateTaskScheduleInput,
    db_conn: &DatabaseConnection,
) -> AppResult<TaskSchedule> {
    let tx = db_conn.begin().await?;

    tx.with(move |tx| async move {
        let tx = tx.as_ref();
        let now = Local::now();
        let mut task_schedule = TaskSchedule {
            id: Uuid::new_v4(),
            user_id,
            date_spec: input.date_spec.0,
            next_date_to_check: now.date_naive(),
            task_title: input.task_title,
            task_point: input.task_point,
        };

        task_schedule
            .schedule_until((now + Duration::days(30)).date_naive(), tx)
            .await?;

        let task_schedule = entities::task_schedule::Model::try_from(task_schedule)?
            .into_active_model()
            .insert(tx)
            .await?;

        Ok(task_schedule.try_into()
            .context("bug: the task schedule saved into the database cannot be converted back to internal representation")?)
    })
    .await
}

#[cfg(test)]
mod tests {
    use crate::graphql::{
        task::{list_tasks, Task, TaskStatus},
        DateRange, TaskFilter,
    };

    use super::*;
    use async_graphql::Json;
    use chrono::TimeZone;
    use googletest::prelude::*;
    use testlib::{test_uuid, PgDocker};

    const DEFAULT_USER_UUID: Uuid = test_uuid(314159);

    async fn insert_default_user(db_conn: &DatabaseConnection) {
        entities::users::ActiveModel {
            id: Set(DEFAULT_USER_UUID),
            username: Set("meteor".to_owned()),
        }
        .insert(db_conn)
        .await
        .expect("cannot insert default user");
    }

    #[googletest::test]
    #[tokio::test]
    async fn can_create_task_schedule() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        testlib::time::set_now(chrono::Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());

        create_task_schedule(
            DEFAULT_USER_UUID,
            CreateTaskScheduleInput {
                date_spec: Json(DateSpec::RepeatsWeekly(WeeklySpec {
                    week_days: vec![Weekday::Mon, Weekday::Tue],
                    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // Monday
                    every_n_week: 1,
                })),
                task_title: "recurring task #1".to_owned(),
                task_point: Some(1),
            },
            db_conn,
        )
        .await?;
        create_task_schedule(
            DEFAULT_USER_UUID,
            CreateTaskScheduleInput {
                date_spec: Json(DateSpec::RepeatsWeekly(WeeklySpec {
                    week_days: vec![Weekday::Mon, Weekday::Sun],
                    start_date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(), // Monday
                    every_n_week: 2,
                })),
                task_title: "recurring task #2".to_owned(),
                task_point: None,
            },
            db_conn,
        )
        .await?;
        let task_schedules = list_task_schedules(DEFAULT_USER_UUID, db_conn).await?;

        expect_that!(
            task_schedules,
            unordered_elements_are![
                pat!(TaskSchedule {
                    user_id: eq(DEFAULT_USER_UUID),
                    date_spec: pat!(DateSpec::RepeatsWeekly(pat!(WeeklySpec {
                        week_days: unordered_elements_are![eq(Weekday::Mon), eq(Weekday::Tue)],
                        start_date: eq(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                        every_n_week: eq(1),
                    }))),
                    task_title: eq("recurring task #1".to_owned()),
                    task_point: some(eq(1)),
                }),
                pat!(TaskSchedule {
                    user_id: eq(DEFAULT_USER_UUID),
                    date_spec: pat!(DateSpec::RepeatsWeekly(pat!(WeeklySpec {
                        week_days: unordered_elements_are![eq(Weekday::Mon), eq(Weekday::Sun)],
                        start_date: eq(NaiveDate::from_ymd_opt(2024, 1, 5).unwrap()),
                        every_n_week: eq(2),
                    }))),
                    task_title: eq("recurring task #2".to_owned()),
                    task_point: none(),
                }),
            ]
        );
        Ok(())
    }

    #[googletest::test]
    #[tokio::test]
    async fn when_creating_task_schedule_create_approaching_tasks() -> Result<()> {
        let pg_docker = PgDocker::new().await;
        let db_conn = pg_docker.db_conn();
        insert_default_user(db_conn).await;
        testlib::time::set_now(chrono::Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());

        println!(
            "{:#?}",
            serde_json::to_string(&DateSpec::RepeatsWeekly(WeeklySpec {
                week_days: vec![Weekday::Mon, Weekday::Tue],
                start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // Monday
                every_n_week: 1,
            }))
        );
        create_task_schedule(
            DEFAULT_USER_UUID,
            CreateTaskScheduleInput {
                date_spec: Json(DateSpec::RepeatsWeekly(WeeklySpec {
                    week_days: vec![Weekday::Mon, Weekday::Tue],
                    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // Monday
                    every_n_week: 1,
                })),
                task_title: "recurring task".to_owned(),
                task_point: Some(1),
            },
            db_conn,
        )
        .await?;

        expect_that!(
            list_tasks(
                DEFAULT_USER_UUID,
                TaskFilter {
                    planned_date_range: Some(DateRange {
                        start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                        end: NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
                    })
                },
                db_conn
            )
            .await?,
            unordered_elements_are![
                pat!(Task {
                    title: eq("recurring task"),
                    status: eq(TaskStatus::Active),
                    point: some(eq(1)),
                    planned_on: eq(Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())),
                }),
                pat!(Task {
                    title: eq("recurring task"),
                    status: eq(TaskStatus::Active),
                    point: some(eq(1)),
                    planned_on: eq(Some(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap())),
                }),
            ]
        );
        Ok(())
    }
}
