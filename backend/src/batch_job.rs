use chrono::{Duration, Local, NaiveDate};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, TransactionTrait};

use crate::{entities, graphql::task_schedule::TaskSchedule};

const TASK_SCHEDULE_PAGE_SIZE: u64 = 50;
const SCHEDULE_AHEAD_DAYS: i64 = 14;

pub async fn run_all_schedule(db: &DatabaseConnection) -> anyhow::Result<()> {
    let now_date: NaiveDate = Local::now().date_naive();
    let schedule_until_date = now_date + Duration::days(SCHEDULE_AHEAD_DAYS);
    let mut task_schedule_batches =
        entities::task_schedule::Entity::find().paginate(db, TASK_SCHEDULE_PAGE_SIZE);

    while let Some(batch) = task_schedule_batches.fetch_and_next().await? {
        for task_shedule in batch {
            let mut task_schedule: TaskSchedule = task_shedule.try_into()?;
            let tx = db.begin().await?;
            task_schedule
                .schedule_until(schedule_until_date, &tx)
                .await?;
        }
    }

    Ok(())
}
