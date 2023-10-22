use std::{
    cell::RefCell,
    future::Future,
    ops::DerefMut,
    rc::{Rc, Weak},
};

use async_trait::async_trait;
use futures::future::BoxFuture;
use sqlx::{PgConnection, Postgres, Transaction};

pub(crate) struct TransactionWrapper(*mut Transaction<'static, Postgres>);

unsafe impl Send for TransactionWrapper {}

impl TransactionWrapper {
    pub(crate) fn as_mut(&mut self) -> &mut PgConnection {
        unsafe { &mut *self.0 }
    }
}

pub(crate) async fn with_transaction_2<F, FUT, T, E>(
    mut tx: Transaction<'static, Postgres>,
    f: F,
) -> Result<T, E>
where
    F: FnOnce(TransactionWrapper) -> FUT,
    FUT: Future<Output = Result<T, E>>,
    E: From<sqlx::Error>,
{
    let wrapper = TransactionWrapper(&mut tx as *mut _);

    let result = f(wrapper).await;
    if result.is_ok() {
        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }

    result
}

#[async_trait]
pub(crate) trait TransactionExt: Sized {
    async fn with<T, E, F, FUT>(mut self, f: F) -> Result<T, E>
    where
        T: Send,
        E: From<sqlx::Error> + Send,
        F: Send + FnOnce(TransactionWrapper) -> FUT,
        FUT: Future<Output = Result<T, E>> + Send;
}

#[async_trait]
impl TransactionExt for Transaction<'static, Postgres> {
    async fn with<T, E, F, FUT>(mut self, f: F) -> Result<T, E>
    where
        T: Send,
        E: From<sqlx::Error> + Send,
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
