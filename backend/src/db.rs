use std::{future::Future, ops::Deref};

use sea_orm::DatabaseTransaction;

pub(crate) struct TransactionWrapper(*mut DatabaseTransaction);

unsafe impl Send for TransactionWrapper {}

impl TransactionWrapper {
    pub(crate) fn as_ref(&self) -> &DatabaseTransaction {
        unsafe { &*self.0 }
    }
}

impl Deref for TransactionWrapper {
    type Target = DatabaseTransaction;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[extend::ext]
pub(crate) impl DatabaseTransaction {
    async fn with<T, E, F, FUT>(mut self, f: F) -> Result<T, E>
    where
        T: Send,
        E: From<sea_orm::DbErr> + Send,
        F: Send + FnOnce(TransactionWrapper) -> FUT,
        FUT: Future<Output = Result<T, E>> + Send,
    {
        let wrapper = TransactionWrapper(&mut self as *mut _);
        let result = f(wrapper).await;
        if result.is_ok() {
            self.commit().await?;
        } else {
            self.rollback().await?;
        }
        result
    }

    async fn with_rollback<T, E, F, FUT>(mut self, f: F) -> Result<(T, DatabaseTransaction), E>
    where
        T: Send,
        E: From<sea_orm::DbErr> + Send,
        F: Send + FnOnce(TransactionWrapper) -> FUT,
        FUT: Future<Output = Result<T, E>> + Send,
    {
        let wrapper = TransactionWrapper(&mut self as *mut _);
        let result = f(wrapper).await;

        match result {
            Ok(t) => {
                Ok((t, self))
            }
            Err(e) => {
                self.rollback().await?;
                Err(e)
            }
        }
    }
}

// pub(crate) trait TransactionExt: Sized {
//     async fn with<T, E, F, FUT>(self, f: F) -> Result<T, E>
//     where
//         T: Send,
//         E: From<sea_orm::DbErr> + Send,
//         F: Send + FnOnce(TransactionWrapper) -> FUT,
//         FUT: Future<Output = Result<T, E>> + Send;
// }
//
// impl TransactionExt for DatabaseTransaction {
//     async fn with<T, E, F, FUT>(mut self, f: F) -> Result<T, E>
//     where
//         T: Send,
//         E: From<sea_orm::DbErr> + Send,
//         F: Send + FnOnce(TransactionWrapper) -> FUT,
//         FUT: Future<Output = Result<T, E>> + Send,
//     {
//         let wrapper = TransactionWrapper(&mut self as *mut _);
//         let result = f(wrapper).await;
//         if result.is_ok() {
//             self.commit().await?;
//         } else {
//             self.rollback().await?;
//         }
//         result
//     }
// }
