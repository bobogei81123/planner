use std::future::Future;

use sea_orm::DatabaseTransaction;

pub(crate) struct TransactionWrapper(*mut DatabaseTransaction);

unsafe impl Send for TransactionWrapper {}

impl TransactionWrapper {
    pub(crate) fn as_ref(&self) -> &DatabaseTransaction {
        unsafe { &*self.0 }
    }
}

pub(crate) trait TransactionExt: Sized {
    async fn with<T, E, F, FUT>(self, f: F) -> Result<T, E>
    where
        T: Send,
        E: From<sea_orm::DbErr> + Send,
        F: Send + FnOnce(TransactionWrapper) -> FUT,
        FUT: Future<Output = Result<T, E>> + Send;
}

impl TransactionExt for DatabaseTransaction {
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
}
