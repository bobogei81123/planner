#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(sqlx::Type)]
#[sqlx(type_name = "task_status")]
#[sqlx(rename_all = "lowercase")]
#[derive(async_graphql::Enum)]
pub enum TaskStatus {
    Active,
    Completed,
}
