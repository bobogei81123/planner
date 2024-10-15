use std::sync::Arc;

use sea_orm::{DbErr, TransactionError};
use uuid::Uuid;

pub(crate) mod maybe;
pub(crate) mod task;
pub(crate) mod time;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{typ} with id = {id} is not found")]
    ResourceNotFound { typ: ResourceType, id: Uuid },
    #[error("invalid input: {reason}")]
    InvalidInput { reason: String },
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, strum::Display)]
pub enum ResourceType {
    Task,
}

impl AppError {
    fn task_not_found(id: Uuid) -> Self {
        AppError::ResourceNotFound {
            typ: ResourceType::Task,
            id,
        }
    }

    fn invalid_input(reason: impl Into<String>) -> Self {
        AppError::InvalidInput {
            reason: reason.into(),
        }
    }
}

// #[derive(Debug)]
// pub(crate) enum BadRequestReason {
//     InvalidTaskScheduleSpec,
// RequiredFieldIsNull { field: String },
// }

// impl Display for BadRequestReason {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
// Self::InvalidDateRange => write!(
//     f,
//     "The date range is not valid. \
//      Start date or end date must be given if the other is, \
//      and the end date must be later than the start date."
// ),
//             Self::InvalidTaskScheduleSpec => {
//                 write!(f, "The task schedule date spec JSON is not valid.")
//             }
//             Self::RequiredFieldIsNull { field } => {
//                 write!(f, "Field {field} is a required field, but set to null.")
//             }
//         }
//     }
// }

// impl ErrorExtensions for AppError {
//     fn extend(&self) -> async_graphql::Error {
//         async_graphql::Error::new(self.to_string()).extend_with(|_, e| {
//             use AppError::*;
//             match self {
//                 ResourceNotFound(..) => e.set("code", "NOT_FOUND"),
//                 BadRequest(..) => e.set("code", "BAD_REQUEST"),
//                 Internal(..) => {
//                     e.set("code", "INTERNAL_SERVER_ERROR");
//                 }
//             }
//         })
//     }
// }

impl From<DbErr> for AppError {
    fn from(value: sea_orm::DbErr) -> Self {
        AppError::Internal(value.into())
    }
}

impl From<Arc<DbErr>> for AppError {
    fn from(value: Arc<sea_orm::DbErr>) -> Self {
        AppError::Internal(value.into())
    }
}

impl<E> From<TransactionError<E>> for AppError
where
    E: std::error::Error,
    AppError: From<E>,
{
    fn from(value: TransactionError<E>) -> Self {
        match value {
            TransactionError::Connection(e) => e.into(),
            TransactionError::Transaction(e) => e.into(),
        }
    }
}
